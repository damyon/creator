//! # Creator
//!
//! `creator` is a 3d modelling application using voxels.
//! It compiles to wasm and uses WebGL to render to a browser.

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
mod ocnode;
mod octree;
mod scene;
mod storage;
mod stored_octree;

use crate::graphics::Graphics;
use crate::scene::Scene;
use crate::storage::Storage;
extern crate nalgebra as na;

/// Init the scene for the first time.
#[wasm_bindgen]
pub fn init_scene() -> Result<bool, JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Scene::init_scene();
    Ok(true)
}

/// Get the list of saved scenes.
#[wasm_bindgen]
pub async fn scene_names() -> Result<JsValue, JsValue> {
    let storage = Storage::new();

    let names = storage.list_scenes().await;
    log::debug!("Got scene_names");
    Ok(JsValue::from(names))
}

/// Save the current scene.
#[wasm_bindgen]
pub async fn save_scene() -> Result<JsValue, JsValue> {
    Scene::save_scene().await;
    Ok(JsValue::from(true))
}

/// Load the current scene.
#[wasm_bindgen]
pub async fn load_scene() -> Result<bool, JsValue> {
    Scene::load_scene().await;
    Ok(true)
}

/// Switch from fluid to solid material.
#[wasm_bindgen]
pub async fn toggle_fluid() -> Result<bool, JsValue> {
    Scene::toggle_solid().await;
    Ok(true)
}

/// Switch from solid to fluid material.
#[wasm_bindgen]
pub async fn toggle_solid() -> Result<bool, JsValue> {
    Scene::toggle_fluid().await;
    Ok(true)
}

/// Switch from solid to noise colours.
#[wasm_bindgen]
pub async fn toggle_noise() -> Result<bool, JsValue> {
    Scene::toggle_noise().await;
    Ok(true)
}

/// Switch from noise to solid colours.
#[wasm_bindgen]
pub async fn toggle_smooth() -> Result<bool, JsValue> {
    Scene::toggle_smooth().await;
    Ok(true)
}

/// Delete the current scene.
#[wasm_bindgen]
pub async fn delete_scene() -> Result<bool, JsValue> {
    Scene::delete_scene().await;
    Ok(true)
}

/// Change the name of the scene.
#[wasm_bindgen]
pub fn set_scene_name(name: &str) -> Result<bool, JsValue> {
    Scene::set_scene_name(name.to_string());
    Ok(true)
}

/// Load the default scene when the page loads.
#[wasm_bindgen]
pub async fn load_first_scene() -> Result<JsValue, JsValue> {
    Scene::load_first_scene().await;
    type DynFunc = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;
    let f: DynFunc = Rc::new(RefCell::new(None));
    let outer_f = f.clone();

    let window = web_sys::window().unwrap();
    *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        draw_scene();

        // We choose to render when idle, not as fast as possible so we don't overload the browser.
        window
            .request_idle_callback(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("failed requesting idle callback");
    }) as Box<dyn FnMut()>));

    let window = web_sys::window().unwrap();
    window
        .request_idle_callback(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("failed requesting idle callback");

    Ok(JsValue::from(true))
}

/// Draw a new frame for the current scene.
pub fn draw_scene() {
    if !Scene::throttle() {
        let mut graphics: Graphics = Graphics::new();
        graphics.setup_shaders();
        Scene::process_commands();
        graphics.clear();

        Scene::draw(&graphics);
    }
}

/// Change the selection shape.
#[wasm_bindgen]
pub fn toggle_selection_shape() -> Result<bool, JsValue> {
    Scene::scene_toggle_selection_shape();

    Ok(true)
}

/// Change what colour we use.
#[wasm_bindgen]
pub fn set_material_color(
    red: &str,
    green: &str,
    blue: &str,
    alpha: &str,
) -> Result<bool, JsValue> {
    Scene::set_scene_material_color(red, green, blue, alpha);

    Ok(true)
}
