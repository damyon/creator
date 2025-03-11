use js_sys::Array;
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

    Ok(true)
}

#[wasm_bindgen]
pub async fn scene_names() -> Result<JsValue, JsValue> {
    let storage = Storage::new();

    log::debug!("Get scene_names");
    let names = storage.list_scenes().await;
    log::debug!("Got scene_names");
    let js_names: Array = names.into_iter().map(JsValue::from).collect();

    Ok(JsValue::from(js_names))
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
