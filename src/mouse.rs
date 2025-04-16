
extern crate nalgebra as na;

use na::Point2;

pub struct Mouse {
    pub last_position: Point2<i32>,
    pub is_pressed: bool,
}

impl Mouse {
    pub const fn new() -> Mouse {
        Mouse {
            last_position: Point2::new(0, 0),
            is_pressed: false,
        }
    }
}
