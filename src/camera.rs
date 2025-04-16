
extern crate nalgebra as na;

use na::Point3;

#[derive(Copy, Clone)]
pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
}

impl Camera {
    pub const fn new() -> Camera {
        Camera {
            eye: Point3::new(38.0, 16.0, 40.0),
            target: Point3::new(0.0, 0.0, 0.0),
        }
    }
}
