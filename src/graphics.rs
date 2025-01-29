pub mod graphics {

    use std::cmp::min;

    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::WebGlRenderbuffer;
    use web_sys::WebGlFramebuffer;
    use web_sys::WebGlTexture;
    use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};
    use crate::camera::camera::Camera;
    use crate::drawable::drawable::Drawable;

    extern crate nalgebra_glm as glm;
    extern crate nalgebra as na;

    use na::{Vector3, Isometry3, Perspective3};

    extern crate js_sys;
    pub struct Graphics {
        pub gl: WebGlRenderingContext,
        pub light_program: Option<WebGlProgram>,
        pub camera_program: Option<WebGlProgram>,
        pub shadow_frame_buffer: Option<WebGlFramebuffer>,
        pub shadow_depth_texture: Option<WebGlTexture>,
        pub shadow_render_buffer: Option<WebGlRenderbuffer>
    }

    impl Graphics {

        pub fn new(canvas_id: &str) -> Graphics {
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

            gl.disable(WebGlRenderingContext::DEPTH_TEST);
            gl.enable(WebGlRenderingContext::BLEND);
            gl.blend_func(WebGlRenderingContext::ONE, WebGlRenderingContext::ONE_MINUS_SRC_ALPHA);

            
            Graphics { 
                gl: gl,
                light_program: None,
                camera_program: None,
                shadow_frame_buffer: None,
                shadow_depth_texture: None,
                shadow_render_buffer: None
            }
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

        pub fn use_light_program(&self) {
            self.gl.use_program(self.light_program.as_ref());
        }

        pub fn use_camera_program(&self) {
            self.gl.use_program(self.camera_program.as_ref());
        }

        pub fn setup_shaders(&mut self) {
            let light_vertex_shader_source = "
                attribute vec3 aVertexPosition;
      
                uniform mat4 uPMatrix;
                uniform mat4 uMVMatrix;

                void main (void) {
                    gl_Position = uPMatrix * uMVMatrix * vec4(aVertexPosition, 1.0);
                }
                ";

            let light_fragment_shader_source = "
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

            let light_vertex_shader_opt = self.create_shader(
                WebGlRenderingContext::VERTEX_SHADER,
                light_vertex_shader_source,
            );

            if light_vertex_shader_opt.is_err() {
                panic!("Could not compile light vertex shader {:?}", light_vertex_shader_opt.err());
            }
            let light_fragment_shader_opt = self.create_shader(
                WebGlRenderingContext::FRAGMENT_SHADER,
                light_fragment_shader_source,
            );
            if light_fragment_shader_opt.is_err() {
                panic!("Could not compile light fragment shader {:?}", light_fragment_shader_opt.err());
            }
        
            self.light_program = Some(self.create_program(light_vertex_shader_opt.unwrap().as_ref(), light_fragment_shader_opt.unwrap().as_ref()));

            let camera_vertex_shader_source = "
                attribute vec3 aVertexPosition;
                attribute vec2 aTextureCoord;

                uniform mat4 uPMatrix;
                uniform mat4 uMVMatrix;
                uniform mat4 lightMViewMatrix;
                uniform mat4 lightProjectionMatrix;
                varying highp vec2 vTextureCoord;

                // Used to normalize our coordinates from clip space to (0 - 1)
                // so that we can access the corresponding point in our depth color texture
                const mat4 texUnitConverter = mat4(0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.5, 0.5, 0.5, 1.0);

                varying vec2 vDepthUv;
                varying vec4 shadowPos;
                varying vec4 depthPos;
                varying vec3 worldPos;

                void main (void) {
                    highp vec3 directionalVector = normalize(vec3(0.1, 1, 2));
                    highp vec3 ambientLight = vec3(0.4, 0.4, 0.4);
                    highp vec3 directionalLightColor = vec3(0.3, 0.3, 0.3);

                    gl_Position = uPMatrix * uMVMatrix * vec4(aVertexPosition, 1.0);

                    depthPos = gl_Position;
                    worldPos = aVertexPosition;

                    shadowPos = texUnitConverter * lightProjectionMatrix * lightMViewMatrix * vec4(aVertexPosition, 1.0);

                    vTextureCoord = aTextureCoord;
                }
                ";
        
            let camera_fragment_shader_source = "
                precision mediump float;

                varying vec2 vDepthUv;
                varying vec4 shadowPos;
                varying vec4 depthPos;
                
                varying highp vec2 vTextureCoord;

                uniform sampler2D depthColorTexture;
                uniform sampler2D uSampler;
                uniform vec3 uColor;
                uniform float uCanvasWidth;
                uniform float uCanvasHeight;
                uniform int isWater;
                uniform int isSand;
                varying vec3 worldPos;

                float decodeFloat(vec4 color) {
                    const vec4 bitShift = vec4(
                    1.0 / (256.0 * 256.0 * 256.0),
                    1.0 / (256.0 * 256.0),
                    1.0 / 256.0,
                    1
                    );
                    return dot(color, bitShift);
                }

                void makeKernel(inout vec4 n[9], sampler2D tex, vec2 coord, float width, float height) {
                    float w = 1.0 / width;
                    float h = 1.0 / height;

                    n[0] = texture2D(tex, coord + vec2( -w, -h));
                    n[1] = texture2D(tex, coord + vec2(0.0, -h));
                    n[2] = texture2D(tex, coord + vec2(  w, -h));
                    n[3] = texture2D(tex, coord + vec2( -w, 0.0));
                    n[4] = texture2D(tex, coord);
                    n[5] = texture2D(tex, coord + vec2(  w, 0.0));
                    n[6] = texture2D(tex, coord + vec2( -w, h));
                    n[7] = texture2D(tex, coord + vec2(0.0, h));
                    n[8] = texture2D(tex, coord + vec2(  w, h));
                }

                void main(void) {
                    highp vec4 texelColor = texture2D(uSampler, vTextureCoord);
                    highp vec3 ambientLight = vec3(0.8, 0.8, 0.8);
                    highp vec3 directionalLightColor = vec3(0.2, 0.2, 0.2);
                    vec3 fragmentDepth = shadowPos.xyz;
                    vec3 worldDepth = depthPos.xyz;
                    float stepU = 1.0 / uCanvasWidth;
                    float stepV = 1.0 / uCanvasHeight;
                    float shadowAcneRemover = 0.005;
                    fragmentDepth.z -= shadowAcneRemover;

                    float texelSize = 1.0 / ${this.shadowDepthTextureSize}.0;
                    float amountInLight = 0.0;

                    // Check whether or not the current fragment and the 8 fragments surrounding
                    // the current fragment are in the shadow. We then average out whether or not
                    // all of these fragments are in the shadow to determine the shadow contribution
                    // of the current fragment.
                    // So if 4 out of 9 fragments that we check are in the shadow then we'll say that
                    // this fragment is 4/9ths in the shadow so it'll be a little brighter than something
                    // that is 9/9ths in the shadow.
                    const int blend = 5;
                    float blendLength = float(blend) * 2.0 + 1.0;

                    for (int x = -blend; x <= blend; x++) {
                    for (int y = -blend; y <= blend; y++) {
                        int bigx = 1 * x;
                        int bigy = 1 * y;
                        float texelDepth = decodeFloat(texture2D(depthColorTexture, fragmentDepth.xy + vec2(bigx, bigy) * texelSize));
                        if (fragmentDepth.z < texelDepth) {
                        amountInLight += 1.0;
                        }
                    }
                    }
                    
                    
                    amountInLight /= blendLength * blendLength;

                
                    //amountInLight = min(amountInLight, 1.8);

                    gl_FragColor = vec4(ambientLight * texelColor.rgb + directionalLightColor * amountInLight * uColor, texelColor.a);

                    // Cartoon shader...
                    //gl_FragColor.r = floor(gl_FragColor.r / 0.05) * 0.05;
                    //gl_FragColor.g = floor(gl_FragColor.g / 0.05) * 0.05;
                    //gl_FragColor.b = floor(gl_FragColor.b / 0.05) * 0.05;

                    vec4 n[9];
                    float lineWidth = 0.2;
                    float outlineCutoff = 0.1;
                    makeKernel( n, uSampler, vTextureCoord.st, uCanvasWidth*lineWidth, uCanvasHeight*lineWidth);

                    vec4 sobel_edge_h = n[2] + (2.0*n[5]) + n[8] - (n[0] + (2.0*n[3]) + n[6]);
                    vec4 sobel_edge_v = n[0] + (2.0*n[1]) + n[2] - (n[6] + (2.0*n[7]) + n[8]);
                    vec4 sobel = sqrt((sobel_edge_h * sobel_edge_h) + (sobel_edge_v * sobel_edge_v));
                    vec4 edgeColor = vec4( 1.0 - sobel.rgb, 1.0 );

                    if (edgeColor.r + edgeColor.g + edgeColor.b < outlineCutoff && isWater == 0) {
                    // This adds a dark border on edges like a cartoon - it is a bit noisy.
                    //gl_FragColor = vec4(0.1, 0.1, 0.1, 0.9);
                    }
                    
                }
                ";

            let camera_vertex_shader_opt = self.create_shader(
                WebGlRenderingContext::VERTEX_SHADER,
                camera_vertex_shader_source,
            );

            if camera_vertex_shader_opt.is_err() {
                panic!("Could not compile lcameraight vertex shader {:?}", camera_vertex_shader_opt.err());
            }
            let camera_fragment_shader_opt = self.create_shader(
                WebGlRenderingContext::FRAGMENT_SHADER,
                camera_fragment_shader_source,
            );
            if camera_fragment_shader_opt.is_err() {
                panic!("Could not compile camera fragment shader {:?}", camera_fragment_shader_opt.err());
            }
        
            self.camera_program = Some(self.create_program(camera_vertex_shader_opt.unwrap().as_ref(), camera_fragment_shader_opt.unwrap().as_ref()));

        
        }

        pub fn clear(&self) {
            self.gl.clear_color(0.1, 0.1, 0.8, 0.5);
            self.gl.clear(WebGlRenderingContext::DEPTH_BUFFER_BIT | WebGlRenderingContext::COLOR_BUFFER_BIT);
        }

        pub fn create_shadow_depth_texture(&mut self) {

            self.shadow_frame_buffer = self.gl.create_framebuffer();
            self.gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, self.shadow_frame_buffer.as_ref());
            
            self.shadow_depth_texture = self.gl.create_texture();
            self.gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, self.shadow_depth_texture.as_ref());
            
            self.gl.tex_parameteri(WebGlRenderingContext::TEXTURE_2D, WebGlRenderingContext::TEXTURE_MAG_FILTER, WebGlRenderingContext::NEAREST as i32);
            self.gl.tex_parameteri(WebGlRenderingContext::TEXTURE_2D, WebGlRenderingContext::TEXTURE_MIN_FILTER, WebGlRenderingContext::NEAREST as i32);
            self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(WebGlRenderingContext::TEXTURE_2D, 0, WebGlRenderingContext::RGBA as i32, 8192, 8192, 0, WebGlRenderingContext::RGBA, WebGlRenderingContext::UNSIGNED_BYTE, None);
            
            self.shadow_render_buffer = self.gl.create_renderbuffer();
            self.gl.bind_renderbuffer(WebGlRenderingContext::RENDERBUFFER, self.shadow_render_buffer.as_ref());
            self.gl.renderbuffer_storage(WebGlRenderingContext::RENDERBUFFER, WebGlRenderingContext::DEPTH_COMPONENT16, 8192, 8192);
            
            self.gl.framebuffer_texture_2d(WebGlRenderingContext::FRAMEBUFFER, WebGlRenderingContext::COLOR_ATTACHMENT0, WebGlRenderingContext::TEXTURE_2D, self.shadow_depth_texture.as_ref(), 0);
            self.gl.framebuffer_renderbuffer(WebGlRenderingContext::FRAMEBUFFER, WebGlRenderingContext::DEPTH_ATTACHMENT, WebGlRenderingContext::RENDERBUFFER, self.shadow_render_buffer.as_ref());
            
            // That was enough to create the texture. unbind to restore the default renderer.

            self.gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
            self.gl.bind_renderbuffer(WebGlRenderingContext::RENDERBUFFER, None);
        }
        

        pub fn draw_shadow_map(&self, drawable: & impl Drawable, render_mode: u32, camera: Camera, light: Camera) {
            self.use_light_program();
            self.setup_vertices(&drawable.vertices(), self.light_program.as_ref().expect("fail"));

            let chunk_size: i32 = 30;

            for chunk in 0..(drawable.count_vertices() as i32) / chunk_size {

                let count = min(chunk_size, drawable.count_vertices() as i32 - (chunk * chunk_size));

                self.gl.draw_arrays(
                    render_mode,
                    chunk * chunk_size as i32,
                    count,
                );
            }
        }


        pub fn draw(&mut self, drawable: & impl Drawable, render_mode: u32, camera: Camera, light: Camera) {

            self.setup_vertices(&drawable.vertices(), self.camera_program.as_ref().expect("fail"));

            let color_location = self.gl
                .get_uniform_location(self.camera_program.as_ref().expect("fail"), "u_color")
                .unwrap();
            self.gl.uniform4fv_with_f32_array(Some(&color_location), drawable.color());

            // We want a model / view / projection matrix
            // Compute the matrices
            // Our camera looks toward the point (0.0, 0.0, 0.0).
            // It is located at (2.0, 2.0, 2.0).
            let eye = camera.eye;
            let target = camera.target;
            let view   = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
            
            // This is translation, rotation
            let model      = Isometry3::new(Vector3::from_row_slice(drawable.translation()), Vector3::from_row_slice(drawable.rotation()));
            
            let projection = Perspective3::new(16.0 / 9.0, 3.14 / 2.0, 0.0, 1000.0);
            let model_view_projection = projection.into_inner() * (view * model).to_homogeneous();

            let u_matrix_location = self.gl
                .get_uniform_location(self.camera_program.as_ref().expect("fail"), "u_matrix")
                .unwrap();

            self.gl.uniform_matrix4fv_with_f32_array(Some(&u_matrix_location), false, model_view_projection.as_slice());

            self.gl.line_width(2.0);

            let chunk_size: i32 = 30;

            for chunk in 0..(drawable.count_vertices() as i32) / chunk_size {

                let count = min(chunk_size, drawable.count_vertices() as i32 - (chunk * chunk_size));

                self.gl.draw_arrays(
                    render_mode,
                    chunk * chunk_size as i32,
                    count,
                );
            }

            
            
        }

        pub fn prepare_shadow_frame(&mut self) {
            self.use_light_program();
            
            // Draw to our off screen drawing buffer
            self.gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, self.shadow_frame_buffer.as_ref());
        
            // Set the viewport to our shadow texture's size
            self.gl.viewport(0, 0, 8192, 8192);
            self.clear();
        }

        pub fn finish_shadow_frame(&mut self) {
            self.gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);
        }
    }
    
}