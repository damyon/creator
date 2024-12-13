pub mod scene {

    use std::sync::{Mutex, MutexGuard};

    use crate::camera::camera::Camera;
    use crate::mouse::mouse::Mouse;

    extern crate nalgebra as na;

    use na::{Point3, Point2};

    pub struct Scene {
        camera: Camera,
        mouse: Mouse
    }

    impl Scene {
        fn access() -> MutexGuard<'static, Scene> {
            static GLOBSTATE: Mutex<Scene> = Mutex::new(Scene { camera: Camera::new(), mouse: Mouse::new() });
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

        pub fn camera_target() -> Point3<f32> {
            let scene = Self::access();
            scene.camera.target
        }

        pub fn set_mouse_is_pressed(is_pressed: bool) {
            let mut scene = Self::access();
            scene.mouse.is_pressed = is_pressed
        }

        pub fn mouse_is_pressed() -> bool {
            let  scene = Self::access();
            scene.mouse.is_pressed
        }

        pub fn set_mouse_last_position(last_position: Point2<i32>) {
            let mut scene = Self::access();
            scene.mouse.last_position = last_position
        }

        pub fn mouse_last_position_difference(current_position: Point2<i32>) -> Point2<i32> {
            let  scene = Self::access();
            Point2::new(current_position.x - scene.mouse.last_position.x, current_position.y - scene.mouse.last_position.y)
        }

        pub fn set_camera_target(target: Point3<f32>) {
            let mut scene = Self::access();

            scene.camera.target = target;
        }
    }
}