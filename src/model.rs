


pub mod model {

    use crate::octree::octree::OcTree;

    pub struct Model {
        pub voxels: OcTree,
        pub translation: [f32; 3],
        pub rotation: [f32; 3],
        pub color: [f32; 4],
    }

    use crate::drawable::drawable::Drawable;

    impl Model {
        pub const fn new() -> Model {
            Model {
                voxels: OcTree::new(),
                translation: [0.0; 3],
                rotation: [0.0; 3],
                color: [0.4, 0.4, 0.2, 0.6]
            }
        }
    }

    impl Drawable for Model {
        fn init(&mut self) {
            self.voxels.init();
        }

        fn count_vertices(&self) -> u8 {
            self.voxels.count_vertices()
        }

        fn translation(&self) -> &[f32; 3] {
            &self.translation
        }

        fn color(&self) -> &[f32; 4] {
            &self.color
        }

        fn translate(&mut self, amount: [f32; 3]) {
            self.translation[0] += amount[0];
            self.translation[1] += amount[1];
            self.translation[2] += amount[2];
        }

        fn rotate(&mut self, amount: [f32; 3]) {
            self.rotation[0] += amount[0];
            self.rotation[1] += amount[1];
            self.rotation[2] += amount[2];
        }

        fn rotation(&self) -> &[f32; 3] {
            &self.rotation
        }

        fn vertices(&self) -> &[f32] {
            self.voxels.vertices()
        }
    }
}