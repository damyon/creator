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

    use na::{Isometry3, Matrix4, Orthographic3, Perspective3, Vector3};

    extern crate js_sys;
    pub struct Graphics {
        pub gl: WebGlRenderingContext,
        pub canvas_width: i32,
        pub canvas_height: i32,
        pub camera_program: Option<WebGlProgram>,
        pub light_program: Option<WebGlProgram>,
        pub shadow_frame_buffer: Option<WebGlFramebuffer>,
        pub shadow_depth_texture: Option<WebGlTexture>,
        pub shadow_texture_size: i32,
        pub swap_shaders: bool,
        pub swap_cameras: bool,
        vertex_buffer_limit: i32,
    }

    impl Graphics {
        pub fn new() -> Graphics {
            let canvas_id = "scene";
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

            gl.enable(WebGlRenderingContext::DEPTH_TEST);
            gl.depth_func(WebGlRenderingContext::LEQUAL);
            gl.enable(WebGlRenderingContext::BLEND);
            gl.blend_func(
                WebGlRenderingContext::ONE,
                WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
            );
            gl.enable(WebGlRenderingContext::CULL_FACE);

            Graphics {
                gl,
                canvas_width,
                canvas_height,
                camera_program: None,
                light_program: None,
                shadow_frame_buffer: None,
                shadow_depth_texture: None,
                shadow_texture_size: 4096,
                swap_shaders: false,
                swap_cameras: false,
                vertex_buffer_limit: 2048,
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
            self.gl.attach_shader(&shader_program, vertex_shader);
            self.gl.attach_shader(&shader_program, fragment_shader);
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
            if self.gl.get_extension("OES_texture_float").is_err() {
                panic!("Webgl extension error.");
            }

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
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGlRenderingContext::TEXTURE_2D,  // target
                    0,                                  // level
                    WebGlRenderingContext::RGBA as i32, // internal format
                    self.shadow_texture_size,           // width
                    self.shadow_texture_size,           // height
                    0,                                  // border
                    WebGlRenderingContext::RGBA,        // format
                    WebGlRenderingContext::FLOAT,       // type
                    None,                               // pixels
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
                self.shadow_texture_size,
                self.shadow_texture_size,
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
        }

        pub fn build_light_projection(&self) -> Matrix4<f32> {
            if self.swap_cameras {
                Perspective3::new(
                    self.canvas_width as f32 / self.canvas_height as f32,
                    3.14 / 4.0, // 45 degrees
                    1.0,
                    200.0,
                )
                .into_inner()
            } else {
                Orthographic3::new(-64.0, 64.0, -64.0, 64.0, 1.0, 240.0).into_inner()
            }
        }

        pub fn build_camera_projection(&self) -> Matrix4<f32> {
            if self.swap_cameras {
                Orthographic3::new(-32.0, 32.0, -32.0, 32.0, 0.1, 120.0).into_inner()
            } else {
                Perspective3::new(
                    self.canvas_width as f32 / self.canvas_height as f32,
                    3.14 / 4.0, // 45 degrees
                    1.0,
                    200.0,
                )
                .into_inner()
            }
        }

        pub fn setup_vertices(
            &self,
            vertices: &[f32],
            normals: &[f32],
            shader_program: &WebGlProgram,
            is_camera: bool,
        ) {
            let a_position: u32 = self.gl.get_attrib_location(shader_program, "a_position") as u32;

            let vertex_buffer = self.gl.create_buffer().unwrap();
            self.gl
                .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

            unsafe {
                let vertices_array = js_sys::Float32Array::view(vertices);

                self.gl.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ARRAY_BUFFER,
                    &vertices_array,
                    WebGlRenderingContext::STATIC_DRAW,
                );
            }

            self.gl
                .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
            self.gl.vertex_attrib_pointer_with_i32(
                a_position,
                3,
                WebGlRenderingContext::FLOAT,
                false,
                0,
                0,
            );
            self.gl.enable_vertex_attrib_array(a_position);

            // Normals
            if is_camera {
                let a_normal: u32 = self.gl.get_attrib_location(shader_program, "a_normal") as u32;
                let normal_buffer = self.gl.create_buffer().unwrap();

                self.gl
                    .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&normal_buffer));

                unsafe {
                    let normals_array = js_sys::Float32Array::view(normals);

                    self.gl.buffer_data_with_array_buffer_view(
                        WebGlRenderingContext::ARRAY_BUFFER,
                        &normals_array,
                        WebGlRenderingContext::STATIC_DRAW,
                    );
                }

                self.gl
                    .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&normal_buffer));
                self.gl.vertex_attrib_pointer_with_i32(
                    a_normal,
                    3,
                    WebGlRenderingContext::FLOAT,
                    false,
                    0,
                    0,
                );
                self.gl.enable_vertex_attrib_array(a_normal);
            }
        }

        pub fn setup_shaders(&mut self) {
            self.light_program = Some(self.setup_light_shaders());
            self.camera_program = Some(self.setup_camera_shaders());
            self.create_shadow_depth_texture();
        }

        pub fn setup_light_shaders(&mut self) -> WebGlProgram {
            let vertex_shader_source = "
                attribute vec4 a_position;

                uniform mat4 uPMatrix;
                uniform mat4 uMVMatrix;
                uniform mat4 u_light_PMatrix;
                uniform mat4 u_light_MVMatrix;

                void main (void) {
                    gl_Position = uPMatrix * uMVMatrix * a_position;
                }
            ";
            let fragment_shader_source = "
                precision mediump float;

                float LinearizeDepth(float depth)
                {
                    return depth;
                }

                void main()
                {
                    gl_FragColor = vec4(vec3(LinearizeDepth(gl_FragCoord.z + 0.0005)), 1.0);
                }
                ";

            let vertex_shader_opt =
                self.create_shader(WebGlRenderingContext::VERTEX_SHADER, vertex_shader_source);

            if vertex_shader_opt.is_err() {
                log::error!(
                    "Could not compile light vertex shader: {:?}",
                    vertex_shader_opt.err()
                );
                panic!("Fail");
            }
            let vertex_shader = vertex_shader_opt.expect("Fail");
            let fragment_shader_opt = self.create_shader(
                WebGlRenderingContext::FRAGMENT_SHADER,
                fragment_shader_source,
            );
            if fragment_shader_opt.is_err() {
                log::error!(
                    "Could not compile light fragment shader: {:?}",
                    fragment_shader_opt.err()
                );
                panic!("Fail");
            }
            let fragment_shader = fragment_shader_opt.expect("Fail");

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
                attribute vec3 a_normal;
                uniform mat4 uPMatrix;
                uniform mat4 uMVMatrix;
                uniform mat4 u_light_PMatrix;
                uniform mat4 u_light_MVMatrix;
                varying vec4 positionFromLightPov;
                varying vec3 v_normal;

                void main(void) {
                    // Multiply the position by the matrix.
                    gl_Position = uPMatrix * uMVMatrix * a_position;

                    positionFromLightPov = u_light_PMatrix * u_light_MVMatrix * a_position;
                    v_normal = a_normal;
                }
                ";

            let fragment_shader_source = "
                precision mediump float;
                uniform vec4 u_color;
                uniform int u_shadow_texture_size;
                uniform sampler2D shadowMap;
                varying vec4 positionFromLightPov;
                varying vec4 positionFromLightMV;
                varying vec3 v_normal;

                float rand(vec2 co){
                    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
                }

                void main(void) {
                    float ambientLight = 0.5;
                    vec3 positionFromLightPovInTexture = positionFromLightPov.xyz/positionFromLightPov.w * 0.5 + 0.5;
                    float shadowNess = rand(positionFromLightPovInTexture.xy) / 1.6 + 0.4;
                    shadowNess = 0.0;
                    float texelSize = 2.0 / 4096.0;

                    const int blend = 5;

                    float blendLength = float(blend) * 2.0 + 1.0;
                    blendLength = blendLength * blendLength;

                    for (int x = -blend; x <= blend; x++) {

                        for (int y = -blend; y <= blend; y++) {
                            int bigx = 1 * x;
                            int bigy = 1 * y;
                            float depth = texture2D(shadowMap, positionFromLightPovInTexture.xy + vec2(bigx, bigy) * texelSize).x;

                            // Range for positionFromLightPovInTexture.z is about 0.24 to 0.31
                            //shadow = positionFromLightPovInTexture.z < 0.31;

                            // Range for depthValue is about 0.21 to 0.31
                            // false is black?
                            if (depth < positionFromLightPovInTexture.z) {
                                shadowNess += 1.0;
                            }

                        }
                    }

                    shadowNess /= blendLength;

                    //if (shadow) {
                    //    shadowNess = 0.0;
                    //}
                    // Diffuse
                    vec3 lightDir = normalize(-(vec3(-3.0, -10.0, 5.0)));
                    vec3 normal = normalize(v_normal);
                    float shade = max(dot(normal, lightDir), 0.0);

                    //shadowNess = 0.0;
                    //shade = 0.0;
                    float combined = ambientLight + 0.6 * shade - 0.2 * shadowNess;

                    gl_FragColor = vec4(u_color.rgb * combined, u_color.a);
                }
                ";

            let vertex_shader_opt =
                self.create_shader(WebGlRenderingContext::VERTEX_SHADER, vertex_shader_source);
            if vertex_shader_opt.is_err() {
                log::error!(
                    "Could not compile camera vertex shader: {:?}",
                    vertex_shader_opt.err()
                );
                panic!("Fail");
            }
            let vertex_shader = vertex_shader_opt.expect("Fail");
            let fragment_shader_opt = self.create_shader(
                WebGlRenderingContext::FRAGMENT_SHADER,
                fragment_shader_source,
            );
            if fragment_shader_opt.is_err() {
                log::error!(
                    "Could not compile camera fragment shader: {:?}",
                    fragment_shader_opt.err()
                );
                panic!("Fail");
            }
            let fragment_shader = fragment_shader_opt.expect("Fail");

            let program = self.create_program(&vertex_shader, &fragment_shader);

            self.gl.link_program(&program);

            self.gl.detach_shader(&program, &vertex_shader);
            self.gl.delete_shader(Some(&vertex_shader));
            self.gl.detach_shader(&program, &fragment_shader);
            self.gl.delete_shader(Some(&fragment_shader));

            program
        }

        pub fn use_light_shader(&self) {
            self.gl.disable(WebGlRenderingContext::BLEND);

            if !self.swap_shaders {
                self.gl.use_program(self.light_program.as_ref());
            } else {
                self.gl.use_program(self.camera_program.as_ref());
            }
        }

        pub fn use_camera_shader(&self) {
            self.gl.enable(WebGlRenderingContext::BLEND);
            self.gl.blend_func(WebGlRenderingContext::SRC_ALPHA, WebGlRenderingContext::ONE_MINUS_SRC_ALPHA);

            if !self.swap_shaders {
                self.gl.use_program(self.camera_program.as_ref());
            } else {
                self.gl.use_program(self.light_program.as_ref());
            }
        }

        pub fn clear(&self) {
            self.gl.clear_color(0.1, 0.1, 0.8, 0.5);
            self.gl.clear(
                WebGlRenderingContext::DEPTH_BUFFER_BIT | WebGlRenderingContext::COLOR_BUFFER_BIT,
            );
        }

        pub fn draw_shadow(&self, drawable: &impl Drawable, render_mode: u32, light: Camera) {
            let shader = if self.swap_shaders {
                self.camera_program.as_ref()
            } else {
                self.light_program.as_ref()
            };
            self.use_light_shader();
            self.setup_vertices(
                drawable.vertices(),
                drawable.normals(),
                shader.expect("fail"),
                false,
            );

            // We want a model / view and a projection matrix
            // Compute the matrices
            let eye = light.eye;
            let target = light.target;
            let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

            // This is translation, rotation
            let model = Isometry3::new(
                Vector3::from_row_slice(drawable.translation()),
                Vector3::from_row_slice(drawable.rotation()),
            );

            let projection = self.build_light_projection();
            let model_view = (view * model).to_homogeneous();
            let u_mv_matrix_location = self
                .gl
                .get_uniform_location(shader.expect("fail"), "uMVMatrix");
            if u_mv_matrix_location.is_some() {
                self.gl.uniform_matrix4fv_with_f32_array(
                    Some(&u_mv_matrix_location.expect("Fail")),
                    false,
                    model_view.as_slice(),
                );
            }

            let u_p_matrix_location = self
                .gl
                .get_uniform_location(shader.expect("fail"), "uPMatrix");

            if u_p_matrix_location.is_some() {
                self.gl.uniform_matrix4fv_with_f32_array(
                    Some(&u_p_matrix_location.expect("Fail")),
                    false,
                    projection.as_slice(),
                );
            }

            let chunk_size: i32 = self.vertex_buffer_limit;
            let upper: i32 = ((drawable.count_vertices() as i32) + (chunk_size - 1)) / chunk_size;
            for chunk in 0i32..upper {
                let count = min(
                    chunk_size,
                    drawable.count_vertices() as i32 - (chunk * chunk_size),
                );
                let reduced_count = count / 3;
                self.gl
                    .draw_arrays(render_mode, chunk * chunk_size, reduced_count);
            }
            self.gl.flush();
        }

        pub fn draw(
            &self,
            drawable: &impl Drawable,
            render_mode: u32,
            camera: Camera,
            light: Camera,
        ) {
            let shader = if self.swap_shaders {
                self.light_program.as_ref()
            } else {
                self.camera_program.as_ref()
            };
            self.use_camera_shader();
            self.setup_vertices(
                drawable.vertices(),
                drawable.normals(),
                shader.expect("fail"),
                true,
            );

            let color_location_opt = self
                .gl
                .get_uniform_location(shader.expect("fail"), "u_color");
            if color_location_opt.is_some() {
                self.gl
                    .uniform4fv_with_f32_array(color_location_opt.as_ref(), drawable.color());
            }

            let u_shadow_texture_size_location_opt = self
                .gl
                .get_uniform_location(shader.expect("fail"), "u_shadow_texture_size");
            if u_shadow_texture_size_location_opt.is_some() {
                self.gl.uniform1i(
                    u_shadow_texture_size_location_opt.as_ref(),
                    self.shadow_texture_size,
                );
            }

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

            let projection_matrix = self.build_camera_projection();
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
                projection_matrix.as_slice(),
            );

            // Repeat these shenanigans for the light matrices.
            let light_eye = light.eye;
            let light_target = light.target;
            let light_view = Isometry3::look_at_rh(&light_eye, &light_target, &Vector3::y());

            // This is translation, rotation
            let light_projection_matrix = self.build_light_projection();
            let light_model_view = (light_view * model).to_homogeneous();

            let u_light_mv_matrix_location = self
                .gl
                .get_uniform_location(shader.expect("fail"), "u_light_MVMatrix");

            if u_light_mv_matrix_location.is_some() {
                self.gl.uniform_matrix4fv_with_f32_array(
                    Some(&u_light_mv_matrix_location.expect("Fail")),
                    false,
                    light_model_view.as_slice(),
                );
            }

            let u_light_p_matrix_location = self
                .gl
                .get_uniform_location(shader.expect("fail"), "u_light_PMatrix");

            if u_light_p_matrix_location.is_some() {
                self.gl.uniform_matrix4fv_with_f32_array(
                    Some(&u_light_p_matrix_location.expect("Fail")),
                    false,
                    light_projection_matrix.as_slice(),
                );
            }

            self.gl.line_width(2.0);

            let chunk_size: i32 = self.vertex_buffer_limit;
            let upper = ((drawable.count_vertices() as i32) + (chunk_size - 1)) / chunk_size;

            for chunk in 0..upper {
                let count = min(
                    chunk_size,
                    drawable.count_vertices() as i32 - (chunk * chunk_size),
                );

                let reduced_count = count / 3;
                self.gl.draw_arrays(render_mode, 0, reduced_count);
            }
            self.gl.flush();
        }

        pub fn prepare_shadow_frame(&self) {
            self.use_light_shader();

            // Draw to our off screen drawing buffer
            self.gl.bind_framebuffer(
                WebGlRenderingContext::FRAMEBUFFER,
                self.shadow_frame_buffer.as_ref(),
            );

            // Set the viewport to our shadow texture's size
            self.gl
                .viewport(0, 0, self.shadow_texture_size, self.shadow_texture_size);
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
            self.gl
                .bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);

            self.gl
                .viewport(0, 0, self.canvas_width, self.canvas_height);
            self.gl.clear_color(0.5, 0.5, 0.7, 1.0);
            self.gl.clear_depth(1.0);
            self.gl.clear(
                WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
            );
            // Bind the shadow texture
            self.gl.bind_texture(
                WebGlRenderingContext::TEXTURE_2D,
                self.shadow_depth_texture.as_ref(),
            );
            let u_shadow_map = self
                .gl
                .get_uniform_location(self.camera_program.as_ref().expect("Fail"), "shadowMap");
            if u_shadow_map.is_some() {
                self.gl.uniform1i(u_shadow_map.as_ref(), 0);
            }
        }

        pub fn finish_camera_frame(&self) {}
    }
}
