pub mod scene {

    use std::sync::{Mutex, MutexGuard};
    use log::logger;
    use web_sys::{WebGlRenderingContext, WebGlProgram};

    use crate::drawable::drawable::Drawable;
    use crate::grid::grid::Grid;
    use crate::{camera::camera::Camera, cube::cube::Cube};
    use crate::mouse::mouse::Mouse;
    use crate::command::command::Command;
    use crate::command_queue::command_queue::CommandQueue;
    use crate::graphics::graphics::Context;

    extern crate nalgebra as na;
    extern crate nalgebra_glm as glm;

    use na::{Point2, Point3, Vector3};

    pub struct Scene {
        pub camera: Camera,
        mouse: Mouse,
        command_input: CommandQueue,
        selection_cube: Cube,
        grid_xz: Grid,
        grid_xy: Grid,
        grid_yz: Grid,
    }

    impl Scene {
        fn access() -> MutexGuard<'static, Scene> {
            static GLOBSTATE: Mutex<Scene> = Mutex::new(
                Scene { 
                    camera: Camera::new(), 
                    mouse: Mouse::new(), 
                    command_input: CommandQueue::new() ,
                    selection_cube: Cube::new(),
                    grid_xz: Grid::new(),
                    grid_xy: Grid::new(),
                    grid_yz: Grid::new(),
                }
            );
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

        pub fn queue_command(command: Command) {
            let mut scene = Self::access();

            scene.command_input.queue_command(command);
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

        pub fn init_scene() {
            let mut scene = Self::access();
            scene.init();
        }

        pub fn init(&mut self) {
            self.selection_cube.init();
            self.grid_xz.init();
            self.grid_xy.init();
            self.grid_xy.rotate([(90.0 as f32).to_radians(), 0.0, 0.0]);
            self.grid_yz.init();
            self.grid_yz.rotate([0.0, (90.0 as f32).to_radians(), 0.0]);
        }

        pub fn draw(context: Context, shader: &WebGlProgram) {
            let  scene = Self::access();
            log::info!("Draw scene");
                
            
            let yellow = vec![0.4, 0.4, 0.2, 0.6];
            context.draw(scene.selection_cube, shader, WebGlRenderingContext::TRIANGLES, yellow, scene.camera);
            let white = vec![1.0, 1.0, 1.0, 0.4];
            context.draw(scene.grid_xz, shader, WebGlRenderingContext::LINES, white.clone(), scene.camera);
            context.draw(scene.grid_xy, shader, WebGlRenderingContext::LINES, white.clone(), scene.camera);
            context.draw(scene.grid_yz, shader, WebGlRenderingContext::LINES, white.clone(), scene.camera);
    
        }
    }
}