use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
extern crate js_sys;

mod drawable;
mod graphics;
mod grid;


use crate::drawable::drawable::Drawable;
use crate::graphics::graphics::Context;
use crate::grid::grid::Grid;

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
    let context: Context = Context::new(canvas_id);
    let shader_program: WebGlProgram = context.setup_shaders();

    let mut grid = Grid{ scale: 4, ..Default::default() };

    grid.init();

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