pub mod storage {
    use crate::stored_octree::stored_octree::StoredOctree;
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

        pub async fn save(self: Self, data: StoredOctree) {
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

        pub async fn delete_scene(self: Self, name: String) {
            let db = Database::open("creation")
                .with_version(1u8)
                .await
                .expect("Database could not open");

            log::debug!("delete_scene The DB was loaded");
            let transaction = db
                .transaction("scenes")
                .with_mode(TransactionMode::Readwrite)
                .build()
                .expect("Transaction could not start");

            log::debug!("delete_scene The transaction was started");
            let store = transaction
                .object_store("scenes")
                .expect("Could not get object store");

            log::debug!("delete_scene We got the store");
            _ = store.delete(name).await.expect("Was not deleted");
            log::debug!("delete_scene We loaded the nuts");
            _ = transaction.commit().await;
            log::debug!("delete_scene We committed the transaction");
        }

        pub async fn load_scene(self: Self, name: String) -> Option<StoredOctree> {
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

            log::debug!("load_scene The DB was loaded");
            let transaction = db
                .transaction("scenes")
                .with_mode(TransactionMode::Readonly)
                .build()
                .expect("Transaction could not start");

            log::debug!("load_scene The transaction was started");
            let store = transaction
                .object_store("scenes")
                .expect("Could not get object store");

            log::debug!("load_scene We got the store");
            let serial: Option<StoredOctree> = store
                .get(name)
                .serde()
                .expect("broken")
                .await
                .expect("waited");
            log::debug!("load_scene We loaded the nuts");

            serial
        }

        pub async fn load_first_scene(self: Self) -> Option<StoredOctree> {
            self.load_scene("Default".to_string()).await
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

            log::debug!("We made a DB to get the names");

            let transaction = db
                .transaction("scenes")
                .with_mode(TransactionMode::Readwrite)
                .build()
                .expect("Transaction could not start");

            let store = transaction
                .object_store("scenes")
                .expect("Could not get object store");


            let mut keys = store.get_all_keys().await.expect("Got keys");
            let mut names: Vec<String> = vec![];
            let mut name = keys.next();
            while name.is_some() {
                names.push(name.expect("has value").expect("is not error"));
                name = keys.next()
            }

            names.clone()
        }
    }
}
