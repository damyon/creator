use crate::cube::Cube;
use crate::ocnode::Ocnode;
use crate::stored_octree::StoredOctree;

#[derive(Clone)]
pub struct Octree {
    pub name: String,
    root: Ocnode,
    depth: u32,
}

impl Octree {
    pub const fn new() -> Octree {
        Octree {
            name: String::new(),
            root: Ocnode::new(),
            depth: 1,
        }
    }

    pub fn active_nodes(&self) -> Vec<Ocnode> {
        self.root.active_nodes()
    }

    pub fn clear(&mut self) {
        self.root.clear();
    }

    pub fn optimize(&mut self, camera_eye: [f32; 3]) {
        self.root.optimize(camera_eye);
    }

    pub fn init(&mut self) {
        // The LEVELS here is important. It defines the number of sub-divisions
        // so it exponentially increases the number of nodes.
        self.decimate(crate::ocnode::LEVELS);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn load_from_serial(&mut self, source: StoredOctree, camera_eye: [f32; 3]) {
        self.name = source.name;

        self.root.clear();

        for node in source.active_nodes {
            self.root.apply(&node);
        }
        self.root.optimize(camera_eye);
    }

    pub fn drawables(&mut self) -> Vec<Cube> {
        self.root.drawables()
    }

    pub fn decimate(&mut self, sub_division_level: u32) {
        self.depth = sub_division_level;
        self.root.decimate(sub_division_level);
    }

    pub fn toggle_voxel(
        &mut self,
        position: [i32; 3],
        value: bool,
        color: [f32; 4],
        camera_eye: [f32; 3],
    ) {
        log::debug!("Toggle voxels in the tree.");
        self.root.toggle_voxel(position, value, color);
        self.root.optimize(camera_eye);
    }

    pub fn prepare(&self) -> StoredOctree {
        log::debug!("Save with name: {:?}", self.name);
        StoredOctree {
            name: String::from(self.name.as_str()),
            active_nodes: self.active_nodes(),
        }
    }

    pub fn all_voxels_active(&self, positions: &Vec<[i32; 3]>) -> bool {
        self.root.all_voxels_active(positions)
    }
}
