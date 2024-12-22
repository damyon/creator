pub mod scene {

    use std::sync::{Mutex, MutexGuard};

    use crate::camera::camera::Camera;
    use crate::mouse::mouse::Mouse;
    use crate::command_queue::command_queue::CommandQueue;

    extern crate nalgebra as na;
    extern crate nalgebra_glm as glm;

    use na::{Point2, Point3, Vector3};

    pub struct Scene {
        camera: Camera,
        mouse: Mouse,
        commands: CommandQueue,
    }

    impl Scene {
        fn access() -> MutexGuard<'static, Scene> {
            static GLOBSTATE: Mutex<Scene> = Mutex::new(Scene { camera: Camera::new(), mouse: Mouse::new(), commands: CommandQueue::new() });
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

        pub fn handle_key_pressed(key: u32) {
            let mut scene = Self::access();

            // W or UP
            if key == 87 || key == 38 {
                // Move up
                scene.camera.eye = Point3::new(scene.camera.eye.x, scene.camera.eye.y + 0.001 as f32, scene.camera.eye.z);
                scene.camera.target = Point3::new(scene.camera.target.x, scene.camera.target.y + 0.001 as f32, scene.camera.target.z);
            }
            // S or X or DOWN
            if key == 83 || key == 88 || key == 40 {
                // Move down
                scene.camera.eye = Point3::new(scene.camera.eye.x, scene.camera.eye.y - 0.001 as f32, scene.camera.eye.z);
                scene.camera.target = Point3::new(scene.camera.target.x, scene.camera.target.y - 0.001 as f32, scene.camera.target.z);
            }
            // A or LEFT
            if key == 65 || key == 37 {
                log::info!("MOVE LEFT");
                let diff = scene.camera.target - scene.camera.eye;
                let blunting = 1000.0;
                //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
                let projection = Vector3::new(diff.z,  0.0, -diff.x) / blunting;
                
                scene.camera.eye += projection;
                scene.camera.target += projection;
            }
            // D or RIGHT
            if key == 68 || key == 39 {
                log::info!("MOVE RIGHT");
                let diff = scene.camera.target - scene.camera.eye;
                let blunting = 1000.0;
                //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
                let projection = Vector3::new(diff.z,  0.0, -diff.x) / blunting;
                
                scene.camera.eye -= projection;
                scene.camera.target -= projection;
            }
        }
    }
}