


pub mod grid {

    extern crate nalgebra as na;
    use na::Vector3;

    pub struct Grid {
        pub scale: u8,
        pub square_count: u8,
        pub vertices_count: u8,
        pub vertices: Vec<f32>,
        pub max_scale: u8,
        pub translation: Vector3<f32>,
        pub rotation: Vector3<f32>
    }


    use crate::drawable::drawable::Drawable;

    impl Default for Grid {
        fn default() -> Grid {
            Grid {
                scale: 1,
                square_count: 1,
                vertices_count: 8,
                vertices: Vec::new(),
                max_scale: 10,
                translation: Vector3::new(0.0, 0.0, 0.0),
                rotation: Vector3::new(0.0, 0.0, 0.0)
            }
        }
    }

    impl Drawable for Grid {
        fn init(&mut self) {
            let row_vertices: [f32; 6] = [
                -1.0, 1.0, 0.0, // top left
                1.0, 1.0, 0.0, // top right
            ];
            let col_vertices: [f32; 6] = [
                -1.0, 1.0, 0.0, // top left
                -1.0, -1.0, 0.0, // bottom left
            ];

            if self.scale > self.max_scale {
                panic!("Scale for grid is out of bounds");
            }

            // We want one pair of vertices for each row +1 and one for each column + 1

            let scale_f = self.scale as f32;
            for row in 0..=self.scale {
                self.vertices.push((row_vertices[0]) * scale_f);
                self.vertices.push(((row as f32) / scale_f * 2.0 - 1.0) * scale_f);
                self.vertices.push((row_vertices[2]) * scale_f);
                self.vertices.push((row_vertices[3]) * scale_f);
                self.vertices.push(((row as f32) / scale_f * 2.0 - 1.0) * scale_f);
                self.vertices.push((row_vertices[5]) * scale_f);
            }

            for col in 0..=self.scale {
                self.vertices.push(((col as f32) / scale_f * 2.0 - 1.0) * scale_f);
                self.vertices.push((col_vertices[1]) * scale_f);
                self.vertices.push((col_vertices[2]) * scale_f);
                self.vertices.push(((col as f32) / scale_f * 2.0 - 1.0) * scale_f);
                self.vertices.push((col_vertices[4]) * scale_f);
                self.vertices.push((col_vertices[5]) * scale_f);
            }
            //log::info!("Our vertices look like this: {:?}", self.vertices);
            self.square_count = self.scale * self.scale;
            self.vertices_count = 2 * (self.scale + 1 + self.scale  + 1);

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