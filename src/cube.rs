


pub mod cube {

    extern crate nalgebra as na;


    #[derive(Copy, Clone)]
    pub struct Cube {
        pub vertices_count: u8,
        pub vertices: [f32; 108],
        pub translation: [f32; 3],
        pub rotation: [f32; 3]
    }

    use crate::drawable::drawable::Drawable;

    impl Cube {
        pub const fn new() -> Cube {
            Cube {
                vertices_count: 108,
                vertices: [0.0; 108],
                translation: [0.0; 3],
                rotation: [0.0; 3]
            }
        }
    }

    impl Drawable for Cube {
        fn init(&mut self) {
            let mut index = 0;
            let mut increment = || -> usize {let result = index; index += 1; result};

            // Bottom
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0;

            self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0;
            // Left
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0;
            
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0;
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0;
            // Right
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0;
            
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0;
           
            // Front
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0;
            
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0;
            // Back
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0;
            
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0;
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0;
            // Top
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0;
            
            self.vertices[increment()] = 0.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0;
            self.vertices[increment()] = 1.0; self.vertices[increment()] = 1.0; self.vertices[increment()] = 0.0;
           

            self.vertices_count = self.vertices.len() as u8;

            self.translation = [0.0; 3];
            self.rotation = [0.0; 3];
        }

        fn count_vertices(&self) -> u8 {
            self.vertices_count
        }

        fn translation(&self) -> &[f32; 3] {
            &self.translation
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
            &self.vertices
        }

    }
}