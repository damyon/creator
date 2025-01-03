use wasm_bindgen::prelude::*;
use web_sys::WebGlProgram;
extern crate js_sys;

mod drawable;
mod graphics;
mod grid;
mod cube;
mod scene;
mod camera;
mod mouse;
mod model;
mod command;
mod command_queue;
mod octree;

use crate::graphics::graphics::Context;
use crate::scene::scene::Scene;
extern crate nalgebra as na;

#[wasm_bindgen]
pub fn init_scene(
    canvas_id: &str
) -> Result<bool, JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    Scene::init_scene(canvas_id);
    Ok(true)
}

#[wasm_bindgen]
pub fn draw_scene(
    canvas_id: &str
) -> Result<bool, JsValue> {
    let context: Context = Context::new(canvas_id);
    let shader_program: WebGlProgram = context.setup_shaders();

    Scene::process_commands();
    
    context.clear();
    Scene::draw(context, &shader_program);
    
    Ok(true)
}