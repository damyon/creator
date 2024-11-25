use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
extern crate js_sys;

mod drawable;
mod graphics;
mod grid;


use crate::drawable::drawable::Drawable;
use crate::graphics::graphics::Context;
use crate::grid::grid::Grid;

#[wasm_bindgen]
pub fn draw_grid(
    canvas_id: &str,
) -> Result<WebGlRenderingContext, JsValue> {
    wasm_logger::init(wasm_logger::Config::default());

    let context: Context = Context::new(canvas_id);
    let shader_program: WebGlProgram = context.setup_shaders();

    let mut grid = Grid{ scale: 2, ..Default::default() };
    
    log::info!("Some info");

    grid.init();

    context.setup_vertices(&grid.vertices, &shader_program);

    context.clear();
    
    context.draw(grid, &shader_program);
    
    

    Ok(context.gl)
}