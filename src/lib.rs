use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};
extern crate js_sys;

mod graphics;

pub fn init_webgl_context(canvas_id: &str) -> Result<WebGlRenderingContext, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    gl.viewport(
        0,
        0,
        canvas.width().try_into().unwrap(),
        canvas.height().try_into().unwrap(),
    );

    Ok(gl)
}

pub fn create_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from_str("Unable to create shader object"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(JsValue::from_str(
            &gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error creating shader".into()),
        ))
    }
}

pub fn setup_shaders(gl: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {
    let vertex_shader_source = "
        attribute vec3 vertexPosition;
        void main(void) {
            gl_Position = vec4(vertexPosition, 1.0);
        }
        ";

    let fragment_shader_source = "
        precision mediump float;
        uniform vec4 fragColor;
        void main(void) {
            gl_FragColor = fragColor;
        }
        ";

    let vertex_shader = create_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        vertex_shader_source,
    )
    .unwrap();
    let fragment_shader = create_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        fragment_shader_source,
    )
    .unwrap();

    let shader_program = gl.create_program().unwrap();
    gl.attach_shader(&shader_program, &vertex_shader);
    gl.attach_shader(&shader_program, &fragment_shader);
    gl.link_program(&shader_program);

    if gl
        .get_program_parameter(&shader_program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        gl.use_program(Some(&shader_program));
        Ok(shader_program)
    } else {
        return Err(JsValue::from_str(
            &gl.get_program_info_log(&shader_program)
                .unwrap_or_else(|| "Unknown error linking program".into()),
        ));
    }
}

pub fn setup_vertices(gl: &WebGlRenderingContext, vertices: &[f32], shader_program: &WebGlProgram) {
    let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };
    let vertex_buffer = gl.create_buffer().unwrap();

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    let vertex_position = gl.get_attrib_location(&shader_program, "vertexPosition");

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(
        vertex_position as u32,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.enable_vertex_attrib_array(vertex_position as u32);
}

#[wasm_bindgen]
pub fn draw_grid(
    canvas_id: &str,
) -> Result<WebGlRenderingContext, JsValue> {
    let context: crate::graphics::graphics::Context = crate::graphics::graphics::build_context(canvas_id).unwrap();
    let shader_program: WebGlProgram = setup_shaders(&context.gl).unwrap();
    let vertices: [f32; 15] = [
        -0.9, 0.9, 0.9, // top left
        -0.9, -0.9, 0.9, // bottom left
        0.9, -0.9, 0.9, // bottom right
        0.9, 0.9, 0.9, // top right
        -0.9, 0.9, 0.9, // top left
    ];

    setup_vertices(&context.gl, &vertices, &shader_program);

    let color = vec![1.0, 1.0, 1.0, 1.0];
    let color_location = context.gl
        .get_uniform_location(&shader_program, "fragColor")
        .unwrap();
    context.gl.uniform4fv_with_f32_array(Some(&color_location), &color);

    context.gl.clear_color(0.4, 0.4, 0.7, 1.0);
    context.gl.clear(WebGlRenderingContext::DEPTH_BUFFER_BIT | WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.gl.line_width(2.0);
    context.gl.draw_arrays(
        WebGlRenderingContext::LINE_STRIP,
        0,
        (vertices.len() / 3) as i32,
    );

    Ok(context.gl)
}