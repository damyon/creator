pub mod drawable {

    extern crate nalgebra as na;

    pub trait Drawable {
        fn init(&mut self);
        fn count_vertices(&self) -> u16;
        fn translation(&self) -> &[f32; 3];
        fn rotation(&self) -> &[f32; 3];
        fn translate(&mut self, amount: [f32; 3]);
        fn rotate(&mut self, amount: [f32; 3]);
        fn vertices(&self) -> &[f32];
        fn normals(&self) -> &[f32];
        fn color(&self) -> &[f32; 4];
    }
}
