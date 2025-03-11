pub mod storage {
    use crate::octree::octree::StoredOcTree;
    use indexed_db_futures::database::Database;
    use indexed_db_futures::transaction::TransactionMode;
    use indexed_db_futures::{prelude::*, KeyPath};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct UserRef {
        id: u32,
        name: String,
    }

    pub struct Storage {
        _noop: f32,
    }

    impl Storage {
        pub fn new() -> Storage {
            Storage { _noop: 0.0 }
        }

        pub async fn save(self: Self, data: StoredOcTree) {
            let db = Database::open("creation")
                .with_version(1u8)
                .with_on_upgrade_needed(|_event, db| {
                    let _create = db
                        .create_object_store("scenes")
                        .with_auto_increment(true)
                        .with_key_path(KeyPath::One("name"))
                        .build()?;

                    Ok(())
                })
                .await
                .expect("Database could not open");

            log::debug!("We made a DB");

            // Populate some data
            let transaction = db
                .transaction("scenes")
                .with_mode(TransactionMode::Readwrite)
                .build()
                .expect("Transaction could not start");

            let store = transaction
                .object_store("scenes")
                .expect("Could not get object store");

            _ = store.put(data).serde();

            _ = transaction.commit().await;
        }

        pub async fn list_scenes(self: Self) -> Vec<String> {
            let db = Database::open("creation")
                .with_version(1u8)
                .with_on_upgrade_needed(|_event, db| {
                    let _create = db
                        .create_object_store("scenes")
                        .with_auto_increment(true)
                        .with_key_path(KeyPath::One("name"))
                        .build()?;

                    Ok(())
                })
                .await
                .expect("Database could not open");

            log::debug!("We made a DB");

            // Populate some data
            let transaction = db
                .transaction("scenes")
                .with_mode(TransactionMode::Readwrite)
                .build()
                .expect("Transaction could not start");

            let store = transaction
                .object_store("scenes")
                .expect("Could not get object store");

            let data = UserRef {
                id: 4,
                name: "Toby".into(),
            };
            _ = store.put(data).serde();

            _ = transaction.commit().await;

            let transaction = db
                .transaction("scenes")
                .with_mode(TransactionMode::Readwrite)
                .build()
                .expect("Transaction could not start");

            let store = transaction
                .object_store("scenes")
                .expect("Could not get object store");

            let cursor_opt = store.open_cursor().await.expect("Got a cursor");
            let mut names: Vec<String> = vec![];

            if cursor_opt.is_some() {
                let mut cursor = cursor_opt.unwrap();
                // This should loop.
                let mut next: Option<String> = cursor.next_key().await.expect("odd");
                // let mut next: Option<String> = cursor.next_record().await.expect("At least one");
                while next.is_some() {
                    names.push(next.unwrap().to_string());
                    next = cursor.next_key().await.expect("At least one");
                }
            }

            names
            /*let window: Window = web_sys::window().expect("no global `window` exists");
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
                        JsValue::from_str("a fing").as_ref(),
                        JsValue::from_str("Default").as_ref(),
                    )
                    .expect("Could not store default value");
            }) as Box<dyn FnOnce(Event)>);

            open_request
                .set_onupgradeneeded(Some(upgrade_required.into_js_value().dyn_ref().unwrap()));

            open_request.set_onsuccess(Some(open_success.into_js_value().dyn_ref().unwrap()));

            vec![]*/
        }
    }
}
