use wasm_bindgen::prelude::*;
extern crate js_sys;

mod camera;
mod command;
mod command_queue;
mod cube;
mod drawable;
mod graphics;
mod grid;
mod model;
mod mouse;
mod octree;
mod scene;
mod storage;

use crate::graphics::graphics::Graphics;
use crate::scene::scene::Scene;
use crate::storage::storage::Storage;
extern crate nalgebra as na;

#[wasm_bindgen]
pub fn init_scene(canvas_id: &str) -> Result<bool, JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    Scene::init_scene(canvas_id);

    let storage: Storage = Storage::new();
    storage.list_scenes();
    Ok(true)
}

#[wasm_bindgen]
pub fn first_scene_name() -> Result<String, JsValue> {
    let storage = Storage::new();
    Ok(storage.first_scene_name())
}

#[wasm_bindgen]
pub fn draw_scene(canvas_id: &str) -> Result<bool, JsValue> {
    let mut graphics: Graphics = Graphics::new(canvas_id);
    graphics.setup_shaders();

    Scene::process_commands();

    graphics.clear();
    Scene::draw(&graphics);

    Ok(true)
}

#[wasm_bindgen]
pub fn set_material_color(red: &str, green: &str, blue: &str) -> Result<bool, JsValue> {
    Scene::set_scene_material_color(red, green, blue);

    Ok(true)
}
