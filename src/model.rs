pub mod model {
    use crate::cube::cube::Cube;
    use crate::octree::octree::Octree;
    use crate::storage::storage::Storage;

    #[derive(Clone)]
    pub struct Model {
        pub voxels: Octree,
    }

    impl Model {
        pub const fn new() -> Model {
            Model {
                voxels: Octree::new(),
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

        pub async fn delete_scene(&self) {
            let storage = Storage::new();
            if self.voxels.name != "Default" {
                _ = storage.delete_scene(self.voxels.name.to_string()).await;
            }
        }

        pub async fn save(&self) {
            let storage = Storage::new();

            let serial = self.voxels.prepare();
            _ = storage.save(serial).await;
        }
    }
}
