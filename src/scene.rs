pub mod scene {

    use std::sync::{Mutex, MutexGuard};
    use web_sys::{WebGlRenderingContext, WebGlProgram};

    use crate::drawable::drawable::Drawable;
    use crate::grid::grid::Grid;
    use crate::{camera::camera::Camera, cube::cube::Cube};
    use crate::mouse::mouse::Mouse;
    use crate::command::command::{Command, CommandType};
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

        fn rotate_2d(target: Point2<f32>, pivot: Point2<f32>, angle_radians: f32) -> Point2<f32> {

            // Precalculate the cosine
            let angle_sin = f32::sin(angle_radians);
            let angle_cos = f32::cos(angle_radians);
            
    
            // Subtract the pivot from the target
            let focused = target - pivot;
            // Rotate
            let rotated = Point2::new(focused.x * angle_cos - focused.y * angle_sin, focused.x * angle_sin + focused.y * angle_cos);
    
            // Add the pivot back
            Point2::new(rotated.x + pivot.x, rotated.y + pivot.y)
        }
    
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

        pub fn queue_command(command: Command) {
            let mut scene = Self::access();

            scene.command_input.queue_command(command);
        }

        pub fn process_commands() {
            let mut scene = Self::access();
            
            let mut command_opt = scene.command_input.next();

            loop {
                match command_opt {
                    Some(command) => {
                        match command.command_type {
                            CommandType::MouseDown => {
                                scene.mouse.is_pressed = true;
                            }
                            CommandType::MouseUp => {
                                scene.mouse.is_pressed = false;
                            }
                            CommandType::MouseMoved => {
                                let current_position = Point2::new(command.data1 as i32, command.data2 as i32);
                                //Scene::handle_mouse_moved(current_position, scene.lock().unwrap());

                                if scene.mouse.is_pressed {
                                    let position_diff = Point2::new(current_position.x - scene.mouse.last_position.x, current_position.y - scene.mouse.last_position.y);
                                    let current_camera_eye = scene.camera.eye;
                                    let current_camera_target = scene.camera.target;
                                    let blunting = 100.0;
                                    let current_camera_eye_2d = Point2::new(current_camera_eye.x, current_camera_eye.z);
                                    let current_camera_target_2d = Point2::new(current_camera_target.x, current_camera_target.z);
                                    // rotate the eye around the target
                                    let adjusted = Self::rotate_2d(current_camera_eye_2d, current_camera_target_2d,  position_diff.x as f32 / blunting);
                    
                                    scene.camera.eye = Point3::new(adjusted.x, current_camera_eye.y, adjusted.y);
                    
                                    // now do the same thing for vertical axis
                                    let current_camera_eye = scene.camera.eye;
                                    let current_camera_eye_2d = Point2::new(current_camera_eye.y, current_camera_eye.z);
                                    let current_camera_target_2d = Point2::new(current_camera_target.y, current_camera_target.z);
                                    // rotate the eye around the target
                                    let adjusted = Self::rotate_2d(current_camera_eye_2d, current_camera_target_2d,  -position_diff.y as f32 / blunting);
                    
                                    scene.camera.eye = Point3::new(current_camera_eye.x, adjusted.x, adjusted.y);
                    
                                }
                                scene.mouse.last_position = current_position;
                                


                            }
                            CommandType::KeyDown => {
                                let key = command.data1;
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
                                    let diff = scene.camera.target - scene.camera.eye;
                                    let blunting = 1000.0;
                                    //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
                                    let projection = Vector3::new(diff.z,  0.0, -diff.x) / blunting;
                                    
                                    scene.camera.eye += projection;
                                    scene.camera.target += projection;
                                }
                                // D or RIGHT
                                if key == 68 || key == 39 {
                                    let diff = scene.camera.target - scene.camera.eye;
                                    let blunting = 1000.0;
                                    //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
                                    let projection = Vector3::new(diff.z,  0.0, -diff.x) / blunting;
                                    
                                    scene.camera.eye -= projection;
                                    scene.camera.target -= projection;
                                }
                            }
                        }
                        
                        command_opt = scene.command_input.next();
                    }
                    None => break
                }
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
            //log::info!("Draw scene");
                
            
            let yellow = vec![0.4, 0.4, 0.2, 0.6];
            context.draw(scene.selection_cube, shader, WebGlRenderingContext::TRIANGLES, yellow, scene.camera);
            let white = vec![1.0, 1.0, 1.0, 0.4];
            context.draw(scene.grid_xz, shader, WebGlRenderingContext::LINES, white.clone(), scene.camera);
            context.draw(scene.grid_xy, shader, WebGlRenderingContext::LINES, white.clone(), scene.camera);
            context.draw(scene.grid_yz, shader, WebGlRenderingContext::LINES, white.clone(), scene.camera);
    
        }
    }
}