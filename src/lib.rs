use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;

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
pub fn init_scene() -> Result<bool, JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    log::debug!("Init scene");
    Scene::init_scene();
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

    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let outer_f = f.clone();

    let window = web_sys::window().unwrap();
    *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw_scene();

        window
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("failed requesting animation frame");
    }) as Box<dyn FnMut()>));

    let window = web_sys::window().unwrap();
    window
        .request_animation_frame(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("failed requesting animation frame");

    Ok(JsValue::from(true))
}

pub fn draw_scene() {
    if !Scene::throttle() {
        let mut graphics: Graphics = Graphics::new();
        graphics.setup_shaders();
        Scene::process_commands();
        graphics.clear();

        Scene::draw(&graphics);
    }

    ()
}

#[wasm_bindgen]
pub fn set_material_color(red: &str, green: &str, blue: &str) -> Result<bool, JsValue> {
    Scene::set_scene_material_color(red, green, blue);

    Ok(true)
}
