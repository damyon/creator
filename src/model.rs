use crate::cube::Cube;
use crate::octree::Octree;
use crate::storage::Storage;

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

    pub fn optimize(&mut self, camera_eye: [f32; 3]) {
        self.voxels.optimize(camera_eye);
    }

    pub fn init(&mut self) {
        self.voxels.init();
    }

    pub fn set_name(&mut self, name: String) {
        self.voxels.set_name(name);
    }

    pub fn toggle_voxel(
        &mut self,
        position: [i32; 3],
        value: bool,
        color: [f32; 4],
        camera_eye: [f32; 3],
    ) {
        log::debug!("Toggle a voxel");
        self.voxels.toggle_voxel(position, value, color, camera_eye);
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
