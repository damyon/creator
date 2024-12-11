pub mod scene {

    use std::sync::{Mutex, MutexGuard};

    use crate::camera::camera::Camera;

    extern crate nalgebra as na;

    use na::Point3;

    pub struct Scene {
        camera: Camera
    }

    impl Scene {
        fn access() -> MutexGuard<'static, Scene> {
            static GLOBSTATE: Mutex<Scene> = Mutex::new(Scene { camera: Camera::new() });
            GLOBSTATE.lock().unwrap()
        }

        pub fn camera_eye() -> Point3<f32> {
            let scene = Self::access();
            scene.camera.eye
        }

        pub fn set_camera_eye(eye: Point3<f32>) {
            let mut scene = Self::access();

            scene.camera.eye = eye;
        }
    }
}