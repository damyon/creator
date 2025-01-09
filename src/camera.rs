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
                eye: Point3::new(8.0, 6.0, 20.0),
                target: Point3::new(1.0, 0.0, 0.0),
            }
        }
    }
}