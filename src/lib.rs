use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlProgram};
extern crate js_sys;

mod drawable;
mod graphics;
mod grid;
mod scene;

use crate::scene::scene::Scene;
use crate::drawable::drawable::Drawable;
use crate::graphics::graphics::Context;
use crate::grid::grid::Grid;
extern crate nalgebra as na;
use na::Vector3;

#[wasm_bindgen]
pub fn draw_grid(
    canvas_id: &str,
    other: &str,
) -> Result<WebGlRenderingContext, JsValue> {
    wasm_logger::init(wasm_logger::Config::default());

    let context: Context = Context::new(canvas_id);
    let shader_program: WebGlProgram = context.setup_shaders();

    let mut grid_xz = Grid{ scale: 4, ..Default::default() };
    let mut grid_xy = Grid{ scale: 4, ..Default::default() };
    let mut grid_yz = Grid{ scale: 4, ..Default::default() };
    

    Scene::print();
    Scene::incr();
    Scene::print();
    log::info!("Some info");
    log::error!("######################{}", other);
    let other_count = other.parse::<i32>().unwrap();
    grid_xz.init();
    grid_xy.init();
    grid_yz.init();

    grid_xy.rotate(Vector3::new((90.0 as f32).to_radians(), 0.0, 0.0));
    grid_yz.rotate(Vector3::new(0.0, (90.0 as f32).to_radians(), 0.0));
    
    context.clear();
    
    context.draw(grid_xz, &shader_program, other_count);
    context.draw(grid_xy, &shader_program, other_count);
    context.draw(grid_yz, &shader_program, other_count);

    Ok(context.gl)
}