


pub mod grid {
    pub struct Grid {
        pub scale: u8,
        pub vertices: [f32; 15]
    }


    use crate::drawable::drawable::Drawable;

    impl Default for Grid {
        fn default() -> Grid {
            Grid {
                scale: 1,
                vertices: [
                    -0.9, 0.9, 0.9, // top left
                    -0.9, -0.9, 0.9, // bottom left
                    0.9, -0.9, 0.9, // bottom right
                    0.9, 0.9, 0.9, // top right
                    -0.9, 0.9, 0.9, // top left
                ]
            }
        }
    }

    impl Drawable for Grid {
        fn init(&mut self) {
            for index in 0..14 {
                self.vertices[index] /= self.scale as f32;
            }
        }

        fn draw(&self) {
        
        }
    }
}