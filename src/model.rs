


pub mod model {

    use crate::cube::cube::Cube;
    use crate::octree::octree::OcTree;

    pub struct Model {
        pub voxels: OcTree
    }

    impl Model {
        pub const fn new() -> Model {
            Model {
                voxels: OcTree::new()
            }
        }

        pub fn drawables(&mut self) -> Vec<Cube> {
            self.voxels.drawables()
        }

        pub fn init(&mut self) {
            self.voxels.init();
        }
    }
}