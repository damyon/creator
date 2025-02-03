pub mod cube {

    #[derive(Copy, Clone)]
    pub struct Cube {
        pub vertices_count: u16,
        pub vertices: [f32; 108],
        pub translation: [f32; 3],
        pub rotation: [f32; 3],
        pub color: [f32; 4],
        pub scale: f32,
        pub floor: f32,
    }

    use crate::drawable::drawable::Drawable;

    impl Cube {
        pub const fn new() -> Cube {
            Cube {
                vertices_count: 108,
                vertices: [0.0; 108],
                translation: [0.0; 3],
                rotation: [0.0; 3],
                color: [0.6, 0.6, 0.2, 0.6],
                scale: 0.95,
                floor: 0.05,
            }
        }
    }

    impl Drawable for Cube {
        fn init(&mut self) {
            let mut index: usize = 0;
            let mut increment = || -> usize {
                let result = index;
                index += 1;
                result
            };
            let scale: f32 = self.scale;
            let floor: f32 = self.floor;

            // Bottom
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;

            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            // Left
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;

            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            // Right
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;

            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;

            // Front
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;

            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            // Back
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;

            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            // Top
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;

            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = scale;
            self.vertices[increment()] = floor;

            self.vertices_count = self.vertices.len() as u16;
        }

        fn count_vertices(&self) -> u16 {
            self.vertices_count
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
            &self.vertices
        }
    }
}
