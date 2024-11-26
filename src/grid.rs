


pub mod grid {
    pub struct Grid {
        pub scale: u8,
        pub square_count: u8,
        pub vertices_count: u8,
        pub vertices: Vec<f32>,
        pub max_scale: u8
    }


    use crate::drawable::drawable::Drawable;

    impl Default for Grid {
        fn default() -> Grid {
            Grid {
                scale: 1,
                square_count: 1,
                vertices_count: 8,
                vertices: Vec::new(),
                max_scale: 10
            }
        }
    }

    impl Drawable for Grid {
        fn init(&mut self) {
            let row_vertices: [f32; 6] = [
                -0.9, 0.9, 0.0, // top left
                0.9, 0.9, 0.0, // top right
            ];
            let col_vertices: [f32; 6] = [
                -0.9, 0.9, 0.0, // top left
                -0.9, -0.9, 0.0, // bottom left
            ];

            if self.scale > self.max_scale {
                panic!("Scale for grid is out of bounds");
            }

            // We want one pair of vertices for each row +1 and one for each column + 1



            for row in 0..=self.scale {
                self.vertices.push(row_vertices[0]);
                self.vertices.push((row as f32) / self.scale as f32 * 1.8 - 0.9);
                self.vertices.push(row_vertices[2]);
                self.vertices.push(row_vertices[3]);
                self.vertices.push((row as f32) / self.scale as f32 * 1.8 - 0.9);
                self.vertices.push(row_vertices[5]);
            }

            for col in 0..=self.scale {
                self.vertices.push((col as f32) / self.scale as f32 * 1.8 - 0.9);
                self.vertices.push(col_vertices[1]);
                self.vertices.push(col_vertices[2]);
                self.vertices.push((col as f32) / self.scale as f32 * 1.8 - 0.9);
                self.vertices.push(col_vertices[4]);
                self.vertices.push(col_vertices[5]);
            }
            log::info!("Our vertices look like this: {:?}", self.vertices);
            self.square_count = self.scale * self.scale;
            self.vertices_count = 2 * (self.scale + 1 + self.scale  + 1);
        }

        fn count_vertices(&self) -> u8 {
            self.vertices_count
        }

    }
}