pub mod drawable {
    
    extern crate nalgebra as na;
    use na::Vector3;

    pub trait Drawable {
        fn init(&mut self);
        fn count_vertices(&self) -> u8;
        fn translation(&self) -> Vector3<f32>;
        fn rotation(&self) -> Vector3<f32>;
        fn translate(&mut self, amount: Vector3<f32>);
        fn rotate(&mut self, amount: Vector3<f32>);
        fn vertices(&self) -> Vec<f32>;
    }
}