pub mod graphics {

    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};

    use crate::drawable::drawable::Drawable;

    extern crate nalgebra_glm as glm;

    extern crate js_sys;
    pub struct Context {
        pub gl: WebGlRenderingContext,
    }

    impl Context {

        pub fn new(canvas_id: &str) -> Context {
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas_element = document.get_element_by_id(canvas_id).unwrap();
            let canvas: web_sys::HtmlCanvasElement = match canvas_element.dyn_into::<web_sys::HtmlCanvasElement>() {
                Ok(canvas) => {
                    canvas
                }
                Err(_) => {
                    panic!("Could not find the canvas element");
                }
            };
            let gl_element = canvas.get_context("webgl").unwrap();
            let gl: WebGlRenderingContext = match gl_element.expect("Found webgl").dyn_into::<WebGlRenderingContext>() {
                Ok(gl) => {
                    gl
                }
                Err(_) => {
                    panic!("Could not get webgl from canvas");
                }
            };
            
            Context { gl: gl }
        }

        pub fn create_shader(
            &self,
            shader_type: u32,
            source: &str,
        ) -> Result<WebGlShader, JsValue> {
            let shader = self.gl
                .create_shader(shader_type)
                .ok_or_else(|| JsValue::from_str("Unable to create shader object"))?;
        
            self.gl.shader_source(&shader, source);
            self.gl.compile_shader(&shader);
        
            if self.gl
                .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
                .as_bool()
                .unwrap_or(false)
            {
                Ok(shader)
            } else {
                Err(JsValue::from_str(
                    &(self.gl).get_shader_info_log(&shader)
                        .unwrap_or_else(|| "Unknown error creating shader".into()),
                ))
            }
        }

        pub fn create_program(&self, vertex_shader: &WebGlShader, fragment_shader: &WebGlShader) -> WebGlProgram {
            let shader_program: WebGlProgram = self.gl.create_program().unwrap();
            self.gl.attach_shader(&shader_program, &vertex_shader);
            self.gl.attach_shader(&shader_program, &fragment_shader);
            self.gl.link_program(&shader_program);
    
            if self.gl
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

        pub fn setup_vertices(&self, vertices: &[f32], shader_program: &WebGlProgram) {
            let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };
            let vertex_buffer = self.gl.create_buffer().unwrap();
        
            self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vertices_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        
            let a_position = self.gl.get_attrib_location(&shader_program, "a_position");
        
            self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
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

        pub fn setup_shaders(&self) -> WebGlProgram {
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
        
            let vertex_shader = self.create_shader(
                WebGlRenderingContext::VERTEX_SHADER,
                vertex_shader_source,
            )
            .unwrap();
            let fragment_shader = self.create_shader(
                WebGlRenderingContext::FRAGMENT_SHADER,
                fragment_shader_source,
            )
            .unwrap();
        
            self.create_program(&vertex_shader, &fragment_shader)
        }

        pub fn clear(&self) {
            self.gl.clear_color(0.4, 0.4, 0.7, 1.0);
            self.gl.clear(WebGlRenderingContext::DEPTH_BUFFER_BIT | WebGlRenderingContext::COLOR_BUFFER_BIT);
        }

        pub fn draw(&self, drawable: impl Drawable, shader_program: &WebGlProgram) {
            let color: Vec<f32> = vec![1.0, 1.0, 1.0, 1.0];
            let color_location = self.gl
                .get_uniform_location(&shader_program, "u_color")
                .unwrap();
            self.gl.uniform4fv_with_f32_array(Some(&color_location), &color);
        
            //let translation: Vec<f32> = vec![0.0, 0.0, -10.0];

            let translation = glm::Vec3::new(40.0, 0.0, -10.0);
            let rotation = glm::Vec3::new(40.0, 0.0, 0.0);
            let scale= glm::Vec3::new(1.0, 1.0, 1.0);


            // We want a model / view / projection matrix
            // Compute the matrices
            let matrix = glm::perspective_fov_lh(3.14 / 4.0, 10.0, 10.0, 1.0, 10.0);
            let modelt = glm::translate(matrix, translation);
    var matrix = m4.projection(gl.canvas.clientWidth, gl.canvas.clientHeight, 400);
    matrix = m4.translate(matrix, translation[0], translation[1], translation[2]);
    matrix = m4.xRotate(matrix, rotation[0]);
    matrix = m4.yRotate(matrix, rotation[1]);
    matrix = m4.zRotate(matrix, rotation[2]);
    matrix = m4.scale(matrix, scale[0], scale[1], scale[2]);

    // Set the matrix.
    gl.uniformMatrix4fv(matrixLocation, false, matrix);

            self.gl.line_width(2.0);
            
            self.gl.draw_arrays(
                WebGlRenderingContext::LINES,
                0,
                (drawable.count_vertices()) as i32,
            );
            log::info!("We are drawing {} vertices", drawable.count_vertices());
        }
    }
    
}