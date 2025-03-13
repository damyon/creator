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
    log::debug!("Init scene");
    Scene::init_scene(canvas_id);
    log::debug!("Init done");
    Ok(true)
}

#[wasm_bindgen]
pub async fn scene_names() -> Result<JsValue, JsValue> {
    let storage = Storage::new();

    let names = storage.list_scenes().await;
    log::debug!("Got scene_names");
    //let js_names: Array = names.into_iter().map(JsValue::from).collect();
    Ok(JsValue::from(names))
    //Ok(JsValue::from(js_names))
}

#[wasm_bindgen]
pub async fn save_scene() -> Result<JsValue, JsValue> {
    Scene::save_scene().await;
    Ok(JsValue::from(true))
}

#[wasm_bindgen]
pub async fn load_scene() -> Result<bool, JsValue> {
    Scene::load_scene().await;
    Ok(true)
}

#[wasm_bindgen]
pub async fn delete_scene() -> Result<bool, JsValue> {
    Scene::delete_scene().await;
    Ok(true)
}

#[wasm_bindgen]
pub fn set_scene_name(name: &str) -> Result<bool, JsValue> {
    Scene::set_scene_name(name.to_string());
    Ok(true)
}

#[wasm_bindgen]
pub async fn load_first_scene() -> Result<JsValue, JsValue> {
    Scene::load_first_scene().await;
    Ok(JsValue::from(true))
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
