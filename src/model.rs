pub mod model {

    use crate::cube::cube::Cube;
    use crate::octree::octree::OcTree;
    use crate::octree::octree::StoredOcTree;
    use crate::storage::storage::Storage;

    pub struct Model {
        pub voxels: OcTree,
    }

    impl Model {
        pub const fn new() -> Model {
            Model {
                voxels: OcTree::new(),
            }
        }

        pub fn drawables(&mut self) -> Vec<Cube> {
            self.voxels.drawables()
        }

        pub fn init(&mut self) {
            self.voxels.init();
        }

        pub fn set_name(&mut self, name: String) {
            self.voxels.set_name(name);
        }

        pub fn toggle_voxel(&mut self, position: [i32; 3], value: bool, color: [f32; 4]) {
            self.voxels.toggle_voxel(position, value, color);
        }

        pub fn all_voxels_active(&self, positions: &Vec<[i32; 3]>) -> bool {
            self.voxels.all_voxels_active(positions)
        }

        pub async fn delete_scene(&mut self) {
            let storage = Storage::new();
            log::debug!("load_first_scene");

            _ = storage.delete_scene(self.voxels.name.to_string()).await;
            self.voxels.clear();
        }

        pub async fn load_scene(&mut self) {
            let storage = Storage::new();
            log::debug!("load_scene");

            let serial: Option<StoredOcTree> =
                storage.load_scene(self.voxels.name.to_string()).await;
            log::debug!("load_scene: We got the scene {:?}", serial.is_some());
            if serial.is_some() {
                log::debug!("load_scene: serial is some");
                self.voxels.load_from_serial(serial.unwrap());
            }
        }

        pub async fn load_first_scene(&mut self) {
            let storage = Storage::new();
            log::debug!("load_first_scene");

            let serial: Option<StoredOcTree> = storage.load_first_scene().await;
            log::debug!("We got the scene {:?}", serial.is_some());
            if serial.is_some() {
                log::debug!("serial is some");
                self.voxels.load_from_serial(serial.unwrap());
            }
        }

        pub async fn save(&self) {
            log::debug!("We save the things");
            let storage = Storage::new();
            log::debug!("Got storage");

            let serial = self.voxels.prepare();
            log::debug!("Prepared the scene");
            _ = storage.save(serial).await;
            log::debug!("We wrote it");
        }
    }
}
