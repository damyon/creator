pub mod camera {
    
    extern crate nalgebra as na;

    use na::Point3;

    #[derive(Copy, Clone)]
    pub struct Camera {
        pub eye: Point3<f32>,
        pub target: Point3<f32>
    }

    impl Camera {

        pub const fn new() -> Camera {
            Camera {
                eye: Point3::new(3.8, 1.0, 7.0),
                target: Point3::new(1.0, 0.0, 0.0),
            }
        }
    }
}