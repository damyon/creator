use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
extern crate js_sys;

mod drawable;
mod graphics;
mod grid;
mod cube;
mod scene;
mod camera;
mod mouse;
mod command;
mod command_queue;

use crate::drawable::drawable::Drawable;
use crate::graphics::graphics::Context;
use crate::grid::grid::Grid;
use crate::cube::cube::Cube;
extern crate nalgebra as na;

#[wasm_bindgen]
pub fn draw_grid(
    canvas_id: &str
) -> Result<WebGlRenderingContext, JsValue> {
    wasm_logger::init(wasm_logger::Config::default());

    let context: Context = Context::new(canvas_id);
    let shader_program: WebGlProgram = context.setup_shaders();

    let mut grid_xz = Grid::new();
    let mut grid_xy = Grid::new();
    let mut grid_yz = Grid::new();

    let mut selection_cube = Cube::new();
    
    grid_xz.init();
    grid_xy.init();
    grid_yz.init();
    selection_cube.init();

    grid_xy.rotate([(90.0 as f32).to_radians(), 0.0, 0.0]);
    grid_yz.rotate([0.0, (90.0 as f32).to_radians(), 0.0]);
    
    context.clear();
    let white = vec![1.0, 1.0, 1.0, 0.4];
    let yellow = vec![0.4, 0.4, 0.2, 0.6];

    context.draw(grid_xz, &shader_program, WebGlRenderingContext::LINES, white.clone());
    context.draw(grid_xy, &shader_program, WebGlRenderingContext::LINES, white.clone());
    context.draw(grid_yz, &shader_program, WebGlRenderingContext::LINES, white.clone());
    context.draw(selection_cube, &shader_program, WebGlRenderingContext::TRIANGLES, yellow.clone());

    Ok(context.gl)
}