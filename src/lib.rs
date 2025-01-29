use wasm_bindgen::prelude::*;
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

use crate::graphics::graphics::Graphics;
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
    let mut graphics: Graphics = Graphics::new(canvas_id);
    graphics.setup_shaders();

    graphics.create_shadow_depth_texture();

    Scene::process_commands();
    
    Scene::draw(&mut graphics);
    
    Ok(true)
}

#[wasm_bindgen]
pub fn set_material_color(red: &str, green: &str, blue: &str) -> Result<bool, JsValue>{

    Scene::set_scene_material_color(red, green, blue);

    Ok(true)
}