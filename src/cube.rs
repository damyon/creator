


pub mod cube {

    extern crate nalgebra as na;
    use na::Vector3;

    pub struct Cube {
        pub vertices_count: u8,
        pub vertices: Vec<f32>,
        pub translation: Vector3<f32>,
        pub rotation: Vector3<f32>
    }


    use crate::drawable::drawable::Drawable;

    impl Default for Cube {
        fn default() -> Cube {
            Cube {
                vertices_count: 24,
                vertices: Vec::new(),
                translation: Vector3::new(0.0, 0.0, 0.0),
                rotation: Vector3::new(0.0, 0.0, 0.0)
            }
        }
    }

    impl Drawable for Cube {
        fn init(&mut self) {
            // Bottom
            self.vertices.push(0.0); self.vertices.push(0.0); self.vertices.push(0.0);
            self.vertices.push(0.0); self.vertices.push(0.0); self.vertices.push(1.0);
            self.vertices.push(1.0); self.vertices.push(0.0); self.vertices.push(0.0);

            self.vertices.push(0.0); self.vertices.push(0.0); self.vertices.push(1.0);
            self.vertices.push(1.0); self.vertices.push(0.0); self.vertices.push(1.0);
            self.vertices.push(1.0); self.vertices.push(0.0); self.vertices.push(0.0);
            // Left
            self.vertices.push(0.0); self.vertices.push(0.0); self.vertices.push(0.0);
            self.vertices.push(0.0); self.vertices.push(1.0); self.vertices.push(0.0);
            self.vertices.push(0.0); self.vertices.push(1.0); self.vertices.push(1.0);
            
            self.vertices.push(0.0); self.vertices.push(0.0); self.vertices.push(0.0);
            self.vertices.push(0.0); self.vertices.push(1.0); self.vertices.push(1.0);
            self.vertices.push(0.0); self.vertices.push(0.0); self.vertices.push(1.0);
            // Right
            self.vertices.push(1.0); self.vertices.push(0.0); self.vertices.push(0.0);
            self.vertices.push(1.0); self.vertices.push(1.0); self.vertices.push(0.0);
            self.vertices.push(1.0); self.vertices.push(1.0); self.vertices.push(1.0);
            
            self.vertices.push(1.0); self.vertices.push(0.0); self.vertices.push(0.0);
            self.vertices.push(1.0); self.vertices.push(1.0); self.vertices.push(1.0);
            self.vertices.push(1.0); self.vertices.push(0.0); self.vertices.push(1.0);
           
            // Front
            self.vertices.push(0.0); self.vertices.push(0.0); self.vertices.push(0.0);
            self.vertices.push(1.0); self.vertices.push(0.0); self.vertices.push(0.0);
            self.vertices.push(1.0); self.vertices.push(1.0); self.vertices.push(0.0);
            
            self.vertices.push(0.0); self.vertices.push(0.0); self.vertices.push(0.0);
            self.vertices.push(1.0); self.vertices.push(1.0); self.vertices.push(0.0);
            self.vertices.push(0.0); self.vertices.push(1.0); self.vertices.push(0.0);
            // Back
            self.vertices.push(0.0); self.vertices.push(0.0); self.vertices.push(1.0);
            self.vertices.push(1.0); self.vertices.push(0.0); self.vertices.push(1.0);
            self.vertices.push(1.0); self.vertices.push(1.0); self.vertices.push(1.0);
            
            self.vertices.push(0.0); self.vertices.push(0.0); self.vertices.push(1.0);
            self.vertices.push(1.0); self.vertices.push(1.0); self.vertices.push(1.0);
            self.vertices.push(0.0); self.vertices.push(1.0); self.vertices.push(1.0);
            // Top
            self.vertices.push(0.0); self.vertices.push(1.0); self.vertices.push(0.0);
            self.vertices.push(0.0); self.vertices.push(1.0); self.vertices.push(1.0);
            self.vertices.push(1.0); self.vertices.push(1.0); self.vertices.push(1.0);
            
            self.vertices.push(0.0); self.vertices.push(1.0); self.vertices.push(0.0);
            self.vertices.push(1.0); self.vertices.push(1.0); self.vertices.push(1.0);
            self.vertices.push(1.0); self.vertices.push(1.0); self.vertices.push(0.0);
           

            self.vertices_count = self.vertices.len() as u8;

            self.translation = Vector3::new(0.0, 0.0, 0.0);
            self.rotation = Vector3::new(0.0, 0.0, 0.0);
        }

        fn count_vertices(&self) -> u8 {
            self.vertices_count
        }

        fn translation(&self) -> Vector3<f32> {
            self.translation
        }

        fn translate(&mut self, amount: Vector3<f32>) {
            self.translation += amount;
        }

        fn rotate(&mut self, amount: Vector3<f32>) {
            self.rotation += amount;
        }

        fn rotation(&self) -> Vector3<f32> {
            self.rotation
        }

        fn vertices(&self) -> Vec<f32> {
            self.vertices.clone()
        }

    }
}