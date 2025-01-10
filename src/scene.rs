pub mod scene {

    use std::sync::{Mutex, MutexGuard};
    use web_sys::{WebGlRenderingContext, WebGlProgram};

    use gloo::events::EventListener;
    use wasm_bindgen::JsCast;
    use crate::drawable::drawable::Drawable;
    use crate::grid::grid::Grid;
    use crate::{camera::camera::Camera, cube::cube::Cube};
    use crate::mouse::mouse::Mouse;
    use crate::command::command::{Command, CommandType};
    use crate::command_queue::command_queue::CommandQueue;
    use crate::graphics::graphics::Context;
    use crate::model::model::Model;

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
        model: Model,
        selection_position: [i32; 3]
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
                    model: Model::new(),
                    selection_position: [0, 0, 0]
                }
            );
            GLOBSTATE.lock().unwrap()
        }

        pub fn queue_command(command: Command) {
            let mut scene = Self::access();

            scene.command_input.queue_command(command);
        }

        pub fn handle_mouse_down(scene: & mut Scene) {
            scene.mouse.is_pressed = true;
        }

        pub fn handle_mouse_up(scene: & mut Scene) {
            scene.mouse.is_pressed = false;
        }

        pub fn handle_mouse_moved(command: &Command, scene: &mut Scene) {
            let current_position = Point2::new(command.data1 as i32, command.data2 as i32);
            
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


        pub fn handle_move_up(scene: &mut Scene) {
            scene.camera.eye = Point3::new(scene.camera.eye.x, scene.camera.eye.y + 0.1 as f32, scene.camera.eye.z);
            scene.camera.target = Point3::new(scene.camera.target.x, scene.camera.target.y + 0.1 as f32, scene.camera.target.z);
        }

        pub fn handle_move_down(scene: &mut Scene) {
            scene.camera.eye = Point3::new(scene.camera.eye.x, scene.camera.eye.y - 0.1 as f32, scene.camera.eye.z);
            scene.camera.target = Point3::new(scene.camera.target.x, scene.camera.target.y - 0.1 as f32, scene.camera.target.z);
        }

        pub fn handle_move_left(scene: &mut Scene) {
            let diff = scene.camera.target - scene.camera.eye;
            let blunting = 10.0;
            //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
            let projection = Vector3::new(diff.z,  0.0, -diff.x) / blunting;
            
            scene.camera.eye += projection;
            scene.camera.target += projection;
        }

        pub fn handle_move_right(scene: &mut Scene) {
            let diff = scene.camera.target - scene.camera.eye;
            let blunting = 10.0;
            //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
            let projection = Vector3::new(diff.z,  0.0, -diff.x) / blunting;
            
            scene.camera.eye -= projection;
            scene.camera.target -= projection;
        }

        pub fn handle_move_forward(scene: &mut Scene) {
            let diff = scene.camera.target - scene.camera.eye;
            let blunting = 10.0;
            let projection = Vector3::new(diff.x,  0.0, diff.z) / blunting;
            
            scene.camera.eye += projection;
            scene.camera.target += projection;
        }

        pub fn handle_move_backward(scene: &mut Scene) {
            let diff = scene.camera.target - scene.camera.eye;
            let blunting = 10.0;
            let projection = Vector3::new(-diff.x,  0.0, -diff.z) / blunting;
            
            scene.camera.eye += projection;
            scene.camera.target += projection;
        }

        pub fn handle_toggle_voxel(scene: &mut Scene) {
            scene.model.toggle_voxel(scene.selection_position);
        }

        pub fn handle_move_selection_left(scene: &mut Scene) {
            scene.selection_cube.translate([-1.0, 0.0, 0.0]);
            scene.selection_position[0] -= 1;
        }

        pub fn handle_move_selection_right(scene: &mut Scene) {
            scene.selection_cube.translate([1.0, 0.0, 0.0]);
            scene.selection_position[0] += 1;
        }

        pub fn handle_move_selection_forward(scene: &mut Scene) {
            scene.selection_cube.translate([0.0, 0.0, 1.0]);
            scene.selection_position[2] += 1;
        }

        pub fn handle_move_selection_backward(scene: &mut Scene) {
            scene.selection_cube.translate([0.0, 0.0, -1.0]);
            scene.selection_position[2] -= 1;
        }
        
        pub fn handle_move_selection_up(scene: &mut Scene) {
            scene.selection_cube.translate([0.0, 1.0, 0.0]);
            scene.selection_position[1] += 1;
        }

        pub fn handle_move_selection_down(scene: &mut Scene) {
            scene.selection_cube.translate([0.0, -1.0, 0.0]);
            scene.selection_position[1] -= 1;
        }

        pub fn handle_mouse_scroll(command: &Command, _scene: &mut Scene) {
            let direction = command.data1;

            if direction > 0 {
                log::info!("Increase the selection");
            } else {
                log::info!("Decrease the selection");
            }
        }

        pub fn handle_key_down(command: &Command, scene: &mut Scene) {
            let key = command.data1;
            
            match key {
                // W or UP
                87|38 => Self::handle_move_up(scene),
                // S or X or DOWN
                83|88|40 => Self::handle_move_down(scene),
                // A or LEFT
                65|37 => Self::handle_move_left(scene),
                // D or RIGHT
                68|39 => Self::handle_move_right(scene),
                // E
                69 => Self::handle_move_forward(scene),
                // C
                67 => Self::handle_move_backward(scene),
                // SPACEBAR
                32 => Self::handle_toggle_voxel(scene),
                // 4
                100 => Self::handle_move_selection_left(scene),
                // 6
                102 => Self::handle_move_selection_right(scene),
                // 2
                98 => Self::handle_move_selection_forward(scene),
                // 8
                104 => Self::handle_move_selection_backward(scene),
                // 9
                105 => Self::handle_move_selection_up(scene),
                // 3
                99 => Self::handle_move_selection_down(scene),
                _ => log::info!("Unhandled key press: {}", key)
            }
        }

        pub fn process_commands() {
            let mut scene = Self::access();
            
            let mut command_opt = scene.command_input.next();

            loop {
                match command_opt {
                    Some(command) => {
                        match command.command_type {
                            CommandType::MouseDown => {
                                Self::handle_mouse_down(&mut scene);
                            }
                            CommandType::MouseUp => {
                                Self::handle_mouse_up(&mut scene);
                            }
                            CommandType::MouseMoved => {
                                Self::handle_mouse_moved(&command, &mut scene);
                            }
                            CommandType::KeyDown => {
                                Self::handle_key_down(&command, &mut scene);
                            }
                            CommandType::MouseScroll => {
                                Self::handle_mouse_scroll(&command, &mut scene);
                            }
                        }
                        
                        command_opt = scene.command_input.next();
                    }
                    None => break
                }
            }
        }

        pub fn init_scene(canvas_id: &str) {
            let mut scene = Self::access();
            scene.init(canvas_id);
        }

        pub fn init(&mut self, canvas_id: &str) {
            self.selection_cube.init();
            self.grid_xz.init();
            self.grid_xy.init();
            self.grid_xy.rotate([(90.0 as f32).to_radians(), 0.0, 0.0]);
            self.grid_yz.init();
            self.grid_yz.rotate([0.0, (90.0 as f32).to_radians(), 0.0]);

            self.model.init();

            let document = web_sys::window().unwrap().document().unwrap();
            let canvas_element = document.get_element_by_id(canvas_id).unwrap();
            let canvas: web_sys::HtmlCanvasElement = match canvas_element.dyn_into::<web_sys::HtmlCanvasElement>() {
                Ok(canvas) => {
                    canvas
                }
                Err(_) => {
                    panic!("Could not find the canvas element");
                }
            };

            let key_down_closure = EventListener::new(&canvas, "keydown", move | event| {
                let key_event = event.clone().dyn_into::<web_sys::KeyboardEvent>().unwrap();
                log::info!("Key down");
                Scene::queue_command(Command {command_type: CommandType::KeyDown, data1: key_event.key_code() as u32, data2: key_event.key_code() as u32});
             });

            key_down_closure.forget();

            let mouse_move_closure = EventListener::new(&canvas, "mousemove", move | event| {
                let move_event = event.clone().dyn_into::<web_sys::MouseEvent>().unwrap();

                // The contents of the closure are only run when the 
                // closure is called by the JS event handler. 
                // The code inside the closures is the only part of this 
                // program that runs repeatedly.
                
                Scene::queue_command(Command {command_type: CommandType::MouseMoved, data1: move_event.offset_x() as u32, data2: move_event.offset_y() as u32});
            });

            mouse_move_closure.forget();

            let wheel_closure = EventListener::new(&canvas, "wheel", move | event | {
                let wheel_event = event.clone().dyn_into::<web_sys::WheelEvent>().unwrap();

                let direction = if wheel_event.delta_y() < 0.0 { 1 as u32 } else { 0 as u32};
                Scene::queue_command(Command {command_type: CommandType::MouseScroll, data1: direction, data2: 1});
            });

            wheel_closure.forget();

            let mouse_down_closure = EventListener::new(&canvas, "mousedown", move | _event| {
                
                Scene::queue_command(Command {command_type: CommandType::MouseDown, data1: 1, data2: 1});
            });

            mouse_down_closure.forget();

            let mouse_up_closure = EventListener::new(&canvas, "mouseup", move | _event| {
                
                Scene::queue_command(Command {command_type: CommandType::MouseUp, data1: 1, data2: 1});
            });

            mouse_up_closure.forget();
        }

        pub fn draw(context: Context, shader: &WebGlProgram) {
            let mut scene = Self::access();
            log::info!("Draw scene");
                
            
            context.draw(&scene.selection_cube, shader, WebGlRenderingContext::TRIANGLES, scene.camera);
            context.draw(&scene.grid_xz, shader, WebGlRenderingContext::LINES, scene.camera);
            context.draw(&scene.grid_xy, shader, WebGlRenderingContext::LINES, scene.camera);
            context.draw(&scene.grid_yz, shader, WebGlRenderingContext::LINES, scene.camera);

            for voxel in scene.model.drawables().iter() {
                context.draw(voxel, shader, WebGlRenderingContext::TRIANGLES, scene.camera);
            }
        }
    }
}