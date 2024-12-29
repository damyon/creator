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
use crate::scene::scene::Scene;
extern crate nalgebra as na;

#[wasm_bindgen]
pub fn init_logger(

) -> Result<bool, JsValue> {
    wasm_logger::init(wasm_logger::Config::default());

    Ok(true)
}

#[wasm_bindgen]
pub fn draw_scene(
    canvas_id: &str
) -> Result<bool, JsValue> {

    log::info!("drw_scene");
            
    let context: Context = Context::new(canvas_id);
    let shader_program: WebGlProgram = context.setup_shaders();
    log::info!("context created");

    Scene::init_scene();
    
    let mut grid_xz = Grid::new();
    log::info!("made one grid");
    
    let mut grid_xy = Grid::new();
    let mut grid_yz = Grid::new();

   // let mut selection_cube = Cube::new();
    
    grid_xz.init();
    log::info!("init one grid");
    grid_xy.init();
    grid_yz.init();

    grid_xy.rotate([(90.0 as f32).to_radians(), 0.0, 0.0]);
    grid_yz.rotate([0.0, (90.0 as f32).to_radians(), 0.0]);
    
    log::info!("clear context");
    context.clear();
    let white = vec![1.0, 1.0, 1.0, 0.4];
    
    Scene::draw(context, &shader_program);
    //context.draw(grid_xz, &shader_program, WebGlRenderingContext::LINES, white.clone());
    //context.draw(grid_xy, &shader_program, WebGlRenderingContext::LINES, white.clone());
    //context.draw(grid_yz, &shader_program, WebGlRenderingContext::LINES, white.clone());
    
    Ok(true)
}