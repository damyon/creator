pub mod drawable {
    pub trait Drawable {
        fn init(&mut self);
        fn count_vertices(&self) -> u8;
    }
}