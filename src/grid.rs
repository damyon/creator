


pub mod grid {
    pub struct Grid {
        pub scale: u8,
        pub square_count: u8,
        pub vertices_count: u8,
        pub vertices: [f32; 1500],
        pub max_scale: u8
    }


    use crate::drawable::drawable::Drawable;

    impl Default for Grid {
        fn default() -> Grid {
            Grid {
                scale: 1,
                square_count: 1,
                vertices_count: 15,
                vertices: [
                    0.0; 1500
                ],
                max_scale: 10
            }
        }
    }

    impl Drawable for Grid {
        fn init(&mut self) {
            let square = [
                -0.9, 0.9, 0.9, // top left
                -0.9, -0.9, 0.9, // bottom left
                0.9, -0.9, 0.9, // bottom right
                0.9, 0.9, 0.9, // top right
                -0.9, 0.9, 0.9, // top left
            ];

            if self.scale > self.max_scale {
                panic!("Scale for grid is out of bounds");
            }
            for row in 0..self.max_scale {
                for col in 0..self.max_scale {
                    for index in 0..15 {
                       let global_index: usize = (row * 15 + col * 15 + index) as usize;

                       self.vertices[global_index] = (square[index as usize] / self.scale as f32 + (row as f32/ self.scale as f32) + (col as f32/ self.scale as f32)) / self.max_scale as f32;
                    }
                }
            }
            log::info!("Our vertices look like this: {:?}", self.vertices);
            self.square_count = self.scale * self.scale;
            self.vertices_count = self.square_count * 15;
        }

        fn count_vertices(&self) -> u8 {
            self.vertices_count
        }

    }
}