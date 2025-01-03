


pub mod model {

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
    }
}