pub mod graphics {

    use std::cmp::min;

    use crate::camera::camera::Camera;
    use crate::drawable::drawable::Drawable;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::WebGlFramebuffer;
    use web_sys::WebGlTexture;
    use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

    extern crate nalgebra as na;
    extern crate nalgebra_glm as glm;

    use na::{Isometry3, Perspective3, Vector3};

    extern crate js_sys;
    pub struct Graphics {
        pub gl: WebGlRenderingContext,
        pub canvas_width: i32,
        pub canvas_height: i32,
        pub camera_program: Option<WebGlProgram>,
        pub light_program: Option<WebGlProgram>,
        pub shadow_frame_buffer: Option<WebGlFramebuffer>,
        pub shadow_depth_texture: Option<WebGlTexture>,
        pub render_depth: bool,
    }

    impl Graphics {
        pub fn new(canvas_id: &str) -> Graphics {
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas_element = document.get_element_by_id(canvas_id).unwrap();
            let canvas: web_sys::HtmlCanvasElement =
                match canvas_element.dyn_into::<web_sys::HtmlCanvasElement>() {
                    Ok(canvas) => canvas,
                    Err(_) => {
                        panic!("Could not find the canvas element");
                    }
                };
            let canvas_width = canvas.client_width();
            let canvas_height = canvas.client_height();
            let gl_element = canvas.get_context("webgl").unwrap();
            let gl: WebGlRenderingContext = match gl_element
                .expect("Found webgl")
                .dyn_into::<WebGlRenderingContext>()
            {
                Ok(gl) => gl,
                Err(_) => {
                    panic!("Could not get webgl from canvas");
                }
            };

            gl.disable(WebGlRenderingContext::DEPTH_TEST);
            gl.enable(WebGlRenderingContext::BLEND);
            gl.blend_func(
                WebGlRenderingContext::ONE,
                WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
            );
            gl.enable(WebGlRenderingContext::CULL_FACE);

            Graphics {
                gl: gl,
                canvas_width: canvas_width,
                canvas_height: canvas_height,
                camera_program: None,
                light_program: None,
                shadow_frame_buffer: None,
                shadow_depth_texture: None,
                render_depth: false,
            }
        }

        pub fn create_shader(
            &self,
            shader_type: u32,
            source: &str,
        ) -> Result<WebGlShader, JsValue> {
            let shader = self
                .gl
                .create_shader(shader_type)
                .ok_or_else(|| JsValue::from_str("Unable to create shader object"))?;

            self.gl.shader_source(&shader, source);
            self.gl.compile_shader(&shader);

            if self
                .gl
                .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
                .as_bool()
                .unwrap_or(false)
            {
                Ok(shader)
            } else {
                Err(JsValue::from_str(
                    &(self.gl)
                        .get_shader_info_log(&shader)
                        .unwrap_or_else(|| "Unknown error creating shader".into()),
                ))
            }
        }

        pub fn create_program(
            &self,
            vertex_shader: &WebGlShader,
            fragment_shader: &WebGlShader,
        ) -> WebGlProgram {
            let shader_program: WebGlProgram = self.gl.create_program().unwrap();
            self.gl.attach_shader(&shader_program, &vertex_shader);
            self.gl.attach_shader(&shader_program, &fragment_shader);
            self.gl.link_program(&shader_program);

            if self
                .gl
                .get_program_parameter(&shader_program, WebGlRenderingContext::LINK_STATUS)
                .as_bool()
                .unwrap_or(false)
            {
                self.gl.use_program(Some(&shader_program));
                shader_program
            } else {
                panic!("Could not compile shaders.")
            }
        }

        pub fn create_shadow_depth_texture(&mut self) {
            self.shadow_frame_buffer = self.gl.create_framebuffer();

            self.gl.bind_framebuffer(
                WebGlRenderingContext::FRAMEBUFFER,
                self.shadow_frame_buffer.as_ref(),
            );

            self.shadow_depth_texture = self.gl.create_texture();

            self.gl.bind_texture(
                WebGlRenderingContext::TEXTURE_2D,
                self.shadow_depth_texture.as_ref(),
            );

            self.gl.tex_parameteri(
                WebGlRenderingContext::TEXTURE_2D,
                WebGlRenderingContext::TEXTURE_MAG_FILTER,
                WebGlRenderingContext::NEAREST as i32,
            );
            self.gl.tex_parameteri(
                WebGlRenderingContext::TEXTURE_2D,
                WebGlRenderingContext::TEXTURE_MIN_FILTER,
                WebGlRenderingContext::NEAREST as i32,
            );

            let result = self
                .gl
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_js_u8_array(
                    WebGlRenderingContext::TEXTURE_2D,
                    0,
                    WebGlRenderingContext::RGBA as i32,
                    8192,
                    8192,
                    0,
                    WebGlRenderingContext::RGBA,
                    WebGlRenderingContext::UNSIGNED_BYTE,
                    None,
                );

            if result.is_err() {
                log::error!("Could not create shadow texture");
                panic!("exit");
            }

            let render_buffer = self.gl.create_renderbuffer();
            self.gl
                .bind_renderbuffer(WebGlRenderingContext::RENDERBUFFER, render_buffer.as_ref());

            self.gl.renderbuffer_storage(
                WebGlRenderingContext::RENDERBUFFER,
                WebGlRenderingContext::DEPTH_COMPONENT16,
                8192,
                8192,
            );
            self.gl.framebuffer_texture_2d(
                WebGlRenderingContext::FRAMEBUFFER,
                WebGlRenderingContext::COLOR_ATTACHMENT0,
                WebGlRenderingContext::TEXTURE_2D,
                self.shadow_depth_texture.as_ref(),
                0,
            );

            self.gl.framebuffer_renderbuffer(
                WebGlRenderingContext::FRAMEBUFFER,
                WebGlRenderingContext::DEPTH_ATTACHMENT,
                WebGlRenderingContext::RENDERBUFFER,
                render_buffer.as_ref(),
            );

            // Unbind now the buffers are created
            self.gl
                .bind_texture(WebGlRenderingContext::TEXTURE_2D, None);

            self.gl
                .bind_renderbuffer(WebGlRenderingContext::RENDERBUFFER, None);

            self.gl
                .bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);
        }

        pub fn setup_vertices(&self, vertices: &[f32], shader_program: &WebGlProgram) {
            let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };
            let vertex_buffer = self.gl.create_buffer().unwrap();

            self.gl
                .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vertices_array,
                WebGlRenderingContext::STATIC_DRAW,
            );

            let a_position = self.gl.get_attrib_location(&shader_program, "a_position");

            self.gl
                .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
            self.gl.vertex_attrib_pointer_with_i32(
                a_position as u32,
                3,
                WebGlRenderingContext::FLOAT,
                false,
                0,
                0,
            );
            self.gl.enable_vertex_attrib_array(a_position as u32);
        }

        pub fn setup_shaders(&mut self) {
            self.light_program = Some(self.setup_light_shaders());
            self.camera_program = Some(self.setup_camera_shaders());
            self.create_shadow_depth_texture();
        }

        pub fn setup_light_shaders(&mut self) -> WebGlProgram {
            let vertex_shader_source = "
                attribute vec3 aVertexPosition;

                      uniform mat4 uPMatrix;
                      uniform mat4 uMVMatrix;

                      void main (void) {
                        gl_Position = uPMatrix * uMVMatrix * vec4(aVertexPosition, 1.0);
                      }
                ";

            let fragment_shader_source = "
                precision mediump float;

                vec4 encodeFloat (float depth) {
                    const vec4 bitShift = vec4(
                        256 * 256 * 256,
                        256 * 256,
                        256,
                        1.0
                    );
                    const vec4 bitMask = vec4(
                        0,
                        1.0 / 256.0,
                        1.0 / 256.0,
                        1.0 / 256.0
                    );
                    vec4 comp = fract(depth * bitShift);
                    comp -= comp.xxyz * bitMask;
                    return comp;
                }

                void main (void) {
                    // Encode the distance into the scene of this fragment.
                    // We'll later decode this when rendering from our camera's
                    // perspective and use this number to know whether the fragment
                    // that our camera is seeing is inside of our outside of the shadow
                    gl_FragColor = encodeFloat(gl_FragCoord.z);
                }
                ";

            let vertex_shader = self
                .create_shader(WebGlRenderingContext::VERTEX_SHADER, vertex_shader_source)
                .unwrap();
            let fragment_shader = self
                .create_shader(
                    WebGlRenderingContext::FRAGMENT_SHADER,
                    fragment_shader_source,
                )
                .unwrap();

            let program = self.create_program(&vertex_shader, &fragment_shader);

            self.gl.link_program(&program);

            self.gl.detach_shader(&program, &vertex_shader);
            self.gl.delete_shader(Some(&vertex_shader));
            self.gl.detach_shader(&program, &fragment_shader);
            self.gl.delete_shader(Some(&fragment_shader));

            program
        }

        pub fn setup_camera_shaders(&mut self) -> WebGlProgram {
            let vertex_shader_source = "
                attribute vec4 a_position;
                uniform mat4 u_matrix;

                void main(void) {
                    // Multiply the position by the matrix.
                    gl_Position = u_matrix * a_position;
                }
                ";

            let fragment_shader_source = "
                precision mediump float;
                uniform vec4 u_color;
                void main(void) {
                    gl_FragColor = u_color;
                }
                ";

            let vertex_shader = self
                .create_shader(WebGlRenderingContext::VERTEX_SHADER, vertex_shader_source)
                .unwrap();
            let fragment_shader = self
                .create_shader(
                    WebGlRenderingContext::FRAGMENT_SHADER,
                    fragment_shader_source,
                )
                .unwrap();

            let program = self.create_program(&vertex_shader, &fragment_shader);

            self.gl.link_program(&program);

            self.gl.detach_shader(&program, &vertex_shader);
            self.gl.delete_shader(Some(&vertex_shader));
            self.gl.detach_shader(&program, &fragment_shader);
            self.gl.delete_shader(Some(&fragment_shader));

            program
        }

        pub fn use_light_shader(&self) {
            self.gl.use_program(self.light_program.as_ref());
        }

        pub fn use_camera_shader(&self) {
            self.gl.use_program(self.camera_program.as_ref());
        }

        pub fn clear(&self) {
            self.gl.clear_color(0.1, 0.1, 0.8, 0.5);
            self.gl.clear(
                WebGlRenderingContext::DEPTH_BUFFER_BIT | WebGlRenderingContext::COLOR_BUFFER_BIT,
            );
        }

        pub fn draw_shadow(&self, drawable: &impl Drawable, render_mode: u32, light: Camera) {
            let shader = self.light_program.as_ref();
            self.use_light_shader();
            self.setup_vertices(&drawable.vertices(), shader.expect("fail"));

            // We want a model / view and a projection matrix
            // Compute the matrices
            // Our camera looks toward the point (0.0, 0.0, 0.0).
            // It is located at (2.0, 2.0, 2.0).
            let eye = light.eye;
            let target = light.target;
            let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

            // This is translation, rotation
            let model = Isometry3::new(
                Vector3::from_row_slice(drawable.translation()),
                Vector3::from_row_slice(drawable.rotation()),
            );

            let projection = Perspective3::new(1.0, 3.14 / 2.0, 0.0, 1000.0).into_inner();
            let model_view = (view * model).to_homogeneous();
            let u_mv_matrix_location = self
                .gl
                .get_uniform_location(shader.expect("fail"), "uMVMatrix")
                .unwrap();

            self.gl.uniform_matrix4fv_with_f32_array(
                Some(&u_mv_matrix_location),
                false,
                model_view.as_slice(),
            );

            let u_p_matrix_location = self
                .gl
                .get_uniform_location(shader.expect("fail"), "uPMatrix")
                .unwrap();

            self.gl.uniform_matrix4fv_with_f32_array(
                Some(&u_p_matrix_location),
                false,
                projection.as_slice(),
            );

            let chunk_size: i32 = 30;

            for chunk in 0..(drawable.count_vertices() as i32) / chunk_size {
                let count = min(
                    chunk_size,
                    drawable.count_vertices() as i32 - (chunk * chunk_size),
                );

                self.gl
                    .draw_arrays(render_mode, chunk * chunk_size as i32, count);
            }
        }

        pub fn draw(&self, drawable: &impl Drawable, render_mode: u32, camera: Camera) {
            let shader = self.camera_program.as_ref();
            self.use_camera_shader();
            self.setup_vertices(&drawable.vertices(), shader.expect("fail"));

            let color_location = self
                .gl
                .get_uniform_location(shader.expect("fail"), "u_color")
                .unwrap();
            self.gl
                .uniform4fv_with_f32_array(Some(&color_location), drawable.color());

            // We want a model / view / projection matrix
            // Compute the matrices
            // Our camera looks toward the point (0.0, 0.0, 0.0).
            // It is located at (2.0, 2.0, 2.0).
            let eye = camera.eye;
            let target = camera.target;
            let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

            // This is translation, rotation
            let model = Isometry3::new(
                Vector3::from_row_slice(drawable.translation()),
                Vector3::from_row_slice(drawable.rotation()),
            );

            let projection = Perspective3::new(
                self.canvas_width as f32 / self.canvas_height as f32,
                3.14 / 2.0,
                0.0,
                1000.0,
            );
            let model_view_projection = projection.into_inner() * (view * model).to_homogeneous();

            let u_matrix_location = self
                .gl
                .get_uniform_location(shader.expect("fail"), "u_matrix")
                .unwrap();

            self.gl.uniform_matrix4fv_with_f32_array(
                Some(&u_matrix_location),
                false,
                model_view_projection.as_slice(),
            );

            self.gl.line_width(2.0);

            let chunk_size: i32 = 30;

            for chunk in 0..(drawable.count_vertices() as i32) / chunk_size {
                let count = min(
                    chunk_size,
                    drawable.count_vertices() as i32 - (chunk * chunk_size),
                );

                self.gl
                    .draw_arrays(render_mode, chunk * chunk_size as i32, count);
            }
        }

        pub fn prepare_shadow_frame(&self) {
            self.use_light_shader();

            // Draw to our off screen drawing buffer
            if self.render_depth {
                self.gl
                    .bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);
            } else {
                self.gl.bind_framebuffer(
                    WebGlRenderingContext::FRAMEBUFFER,
                    self.shadow_frame_buffer.as_ref(),
                );
            }

            // Set the viewport to our shadow texture's size
            self.gl.viewport(0, 0, 8192, 8192);
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
            self.gl.clear_depth(1.0);
            self.gl.clear(
                WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
            );
        }

        pub fn finish_shadow_frame(&self) {
            self.gl
                .bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);
        }

        pub fn prepare_camera_frame(&self) {
            self.use_camera_shader();
            if self.render_depth {
                self.gl.bind_framebuffer(
                    WebGlRenderingContext::FRAMEBUFFER,
                    self.shadow_frame_buffer.as_ref(),
                );
            } else {
                self.gl
                    .bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);
            }
            self.gl
                .viewport(0, 0, self.canvas_width, self.canvas_height);
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
            self.gl.clear_depth(1.0);
            self.gl.clear(
                WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
            );
        }

        pub fn finish_camera_frame(&self) {}
    }
}
