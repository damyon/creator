


pub mod grid {

    #[derive(Copy, Clone)]
    pub struct Grid {
        pub scale: u16,
        pub square_count: u16,
        pub vertices_count: u16,
        pub vertices: [f32; 204],
        pub max_scale: u16,
        pub translation: [f32; 3],
        pub rotation: [f32; 3],
        pub color: [f32; 4]
    }

    use crate::drawable::drawable::Drawable;

    impl Grid {
        pub const fn new() -> Grid {
            Grid {
                scale: 16,
                square_count: 256, // self.scale * self.scale
                vertices_count: 204, // 2 * (6 * (self.scale+1))
                vertices: [0.0; 204],
                max_scale: 20,
                translation: [0.0; 3],
                rotation: [0.0; 3],
                color: [0.2, 0.2, 0.2, 0.1],
            }
        }
    }

    impl Drawable for Grid {
        fn init(&mut self) {
            let mut index = 0;
            let mut increment = || -> usize {let result = index; index += 1; result};

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
                self.vertices[increment()] = row_vertices[0] * scale_f / 2.0;
                self.vertices[increment()] = (-scale_f) / 2.0 + row as f32;
                self.vertices[increment()] = (row_vertices[2]) * scale_f / 2.0;
                self.vertices[increment()] = (row_vertices[3]) * scale_f / 2.0;
                self.vertices[increment()] = (-scale_f) / 2.0 + row as f32;
                self.vertices[increment()] = (row_vertices[5]) * scale_f / 2.0;
            }

            for col in 0..=self.scale {
                self.vertices[increment()] = (-scale_f) / 2.0 + col as f32;
                self.vertices[increment()] = (col_vertices[1]) * scale_f / 2.0;
                self.vertices[increment()] = (col_vertices[2]) * scale_f / 2.0;
                self.vertices[increment()] = (-scale_f) / 2.0 + col as f32;
                self.vertices[increment()] = (col_vertices[4]) * scale_f / 2.0;
                self.vertices[increment()] = (col_vertices[5]) * scale_f / 2.0;
            }
            
            self.square_count = self.scale * self.scale;
            self.vertices_count = 2 * (6 * (self.scale+1));
        }

        fn count_vertices(&self) -> u16 {
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


        fn color(&self) -> &[f32; 4] {
            &self.color
        }

    }
}