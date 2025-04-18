use crate::stored_octree::StoredOctree;
use indexed_db_futures::database::Database;
use indexed_db_futures::transaction::TransactionMode;
use indexed_db_futures::{prelude::*, KeyPath};
use serde::{Deserialize, Serialize};

/// Save to a string.
#[derive(Serialize, Deserialize)]
struct UserRef {
    id: u32,
    name: String,
}

/// We don't use this struct.
pub struct Storage {
    _noop: f32,
}

impl Storage {
    /// Create a new storage.
    pub fn new() -> Storage {
        Storage { _noop: 0.0 }
    }

    /// Save a scene (later in a different thread)
    pub async fn save(self, data: StoredOctree) {
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

    /// Delete a scene.
    pub async fn delete_scene(self, name: String) {
        let db = Database::open("creation")
            .with_version(1u8)
            .await
            .expect("Database could not open");

        let transaction = db
            .transaction("scenes")
            .with_mode(TransactionMode::Readwrite)
            .build()
            .expect("Transaction could not start");

        let store = transaction
            .object_store("scenes")
            .expect("Could not get object store");

        store.delete(name).await.expect("Was not deleted");
        _ = transaction.commit().await;
    }

    /// Load a scene.
    pub async fn load_scene(self, name: String) -> Option<StoredOctree> {
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

        let transaction = db
            .transaction("scenes")
            .with_mode(TransactionMode::Readonly)
            .build()
            .expect("Transaction could not start");

        let store = transaction
            .object_store("scenes")
            .expect("Could not get object store");

        let serial: Option<StoredOctree> = store
            .get(name)
            .serde()
            .expect("broken")
            .await
            .expect("waited");

        serial
    }

    /// Load the default scene.
    pub async fn load_first_scene(self) -> Option<StoredOctree> {
        self.load_scene("Default".to_string()).await
    }

    /// Get a list of saved scenes.
    pub async fn list_scenes(self) -> Vec<String> {
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
