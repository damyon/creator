pub mod drawable {
    pub trait Drawable {
        fn init(&mut self);
        fn draw(&self);
    }
}