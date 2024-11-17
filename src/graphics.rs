pub mod graphics {

    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};

    extern crate js_sys;
    pub struct Context {
        pub gl: WebGlRenderingContext,
    }

    pub fn build_context(canvas_id: &str) -> Result<Context, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas_element = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas_element.dyn_into::<web_sys::HtmlCanvasElement>()?;
        let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();
        
        Ok(Context {
            gl: gl
        })
    }


}