pub mod storage {
    use js_sys::{Array, Promise};
    use std::sync::Mutex;
    use wasm_bindgen::prelude::Closure;
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_futures::spawn_local;
    use web_sys::{Event, IdbTransaction};
    use web_sys::{
        IdbDatabase, IdbFactory, IdbObjectStore, IdbOpenDbRequest, IdbRequest, IdbTransactionMode,
        Window,
    };

    pub struct Storage {
        noop: f32,
    }

    pub static SCENE_NAMES: Mutex<Vec<String>> = Mutex::new(vec![]);

    impl Storage {
        pub fn new() -> Storage {
            Storage { noop: 0.0 }
        }

        pub fn first_scene_name(self: Self) -> String {
            let result = SCENE_NAMES.lock().unwrap();
            log::debug!("Result is {:?}", result[0]);
            result[0].clone()
        }

        pub async fn list_scenes(self: Self) -> Vec<String> {
            let window: Window = web_sys::window().expect("no global `window` exists");
            let factory: IdbFactory = window
                .indexed_db()
                .expect("no global `indexedDB` exists")
                .unwrap()
                .clone();

            let open_request: IdbOpenDbRequest = factory
                .open_with_u32("creator", 1)
                .expect("Failed to open db");

            let request_success = Closure::once(Box::new(move |event: Event| {
                log::debug!("We got a list of keys");
                let target: IdbRequest = event.target().unwrap().dyn_into().unwrap();
                log::debug!("We got a target: {:?}", target.result().unwrap().is_array());
                let scenes: Array = target.result().unwrap().dyn_into().unwrap();
                let mut glob_scenes = SCENE_NAMES.lock().unwrap();
                for (_index, name) in scenes.iter().enumerate() {
                    let name_string: String =
                        name.as_string().expect("Did not get value").to_owned();
                    let b = name_string.to_owned();
                    glob_scenes.push(b);
                }
            }));

            let open_success = Closure::once(Box::new(move |event: Event| {
                let target: IdbOpenDbRequest = event.target().unwrap().dyn_into().unwrap();
                let db: IdbDatabase = target.result().unwrap().dyn_into().unwrap();

                log::debug!("We opened it from listing. What is it? {:?}", db);

                let transaction: IdbTransaction = db
                    .transaction_with_str_and_mode("scenes", IdbTransactionMode::Readonly)
                    .expect("Could not open transaction");
                let object_store: IdbObjectStore = transaction
                    .object_store("scenes")
                    .expect("Could not open object store");

                let request = object_store
                    .get_all_keys()
                    .expect("Could not get scene list");

                request.set_onsuccess(Some(request_success.into_js_value().dyn_ref().unwrap()));
            }) as Box<dyn FnOnce(Event)>);

            let upgrade_required = Closure::once(Box::new(move |event: Event| {
                let target: IdbOpenDbRequest = event.target().unwrap().dyn_into().unwrap();
                let db: IdbDatabase = target.result().unwrap().dyn_into().unwrap();
                log::debug!("We need to build the thing");

                let object_store: IdbObjectStore = db
                    .create_object_store("scenes")
                    .expect("Could not create object store");

                let _index = object_store
                    .create_index_with_str("name", "name")
                    .expect("Could not create index");

                log::debug!("We made a store with an index");

                let _request = object_store
                    .add_with_key(
                        JsValue::from_str("").as_ref(),
                        JsValue::from_str("Default").as_ref(),
                    )
                    .expect("Could not store default value");
            }) as Box<dyn FnOnce(Event)>);

            open_request
                .set_onupgradeneeded(Some(upgrade_required.into_js_value().dyn_ref().unwrap()));

            open_request.set_onsuccess(Some(open_success.into_js_value().dyn_ref().unwrap()));

            open_success.await;
            vec![]
        }
    }
}
