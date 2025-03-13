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

        pub async fn load_first_scene(self: Self) -> Option<StoredOcTree> {
            let db = Database::open("creation")
                .with_version(1u8)
                .await
                .expect("Database could not open");

            log::debug!("load_first_scene The DB was loaded");
            let transaction = db
                .transaction("scenes")
                .with_mode(TransactionMode::Readonly)
                .build()
                .expect("Transaction could not start");

            log::debug!("load_first_scene The transaction was started");
            let store = transaction
                .object_store("scenes")
                .expect("Could not get object store");

            log::debug!("load_first_scene We got the store");
            let serial: Option<StoredOcTree> = store
                .get("Default")
                .serde()
                .expect("broken")
                .await
                .expect("waited");
            log::debug!("load_first_scene We loaded the nuts");

            serial
        }

        /*
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
                while next.is_some() {
                    names.push(next.unwrap().to_string());
                    next = cursor.next_key().await.expect("At least one");
                }
            }

            names
        }*/
    }
}
