pub mod scene {

    use std::cmp::{max, min};
    use std::sync::{Mutex, MutexGuard};
    use web_sys::WebGlRenderingContext;

    use crate::command::command::{Command, CommandType};
    use crate::command_queue::command_queue::CommandQueue;
    use crate::drawable::drawable::Drawable;
    use crate::graphics::graphics::Graphics;
    use crate::grid::grid::Grid;
    use crate::model::model::Model;
    use crate::mouse::mouse::Mouse;
    use crate::{camera::camera::Camera, cube::cube::Cube};
    use gloo::events::EventListener;
    use wasm_bindgen::JsCast;

    extern crate nalgebra as na;
    extern crate nalgebra_glm as glm;

    use na::{Point2, Point3, Vector3};

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum SelectionShape {
        Sphere,
        Cube,
    }

    pub struct Scene {
        pub camera: Camera,
        pub light: Camera,
        mouse: Mouse,
        command_input: CommandQueue,
        selection_cube: Cube,
        grid_xz: Grid,
        model: Model,
        selection_position: [i32; 3],
        selection_radius: u32,
        selection_shape: SelectionShape,
        material_color: [f32; 4],
        initialized: bool,
    }

    impl Scene {
        fn rotate_2d(target: Point2<f32>, pivot: Point2<f32>, angle_radians: f32) -> Point2<f32> {
            // Precalculate the cosine
            let angle_sin = f32::sin(angle_radians);
            let angle_cos = f32::cos(angle_radians);

            // Subtract the pivot from the target
            let focused = target - pivot;
            // Rotate
            let rotated = Point2::new(
                focused.x * angle_cos - focused.y * angle_sin,
                focused.x * angle_sin + focused.y * angle_cos,
            );

            // Add the pivot back
            Point2::new(rotated.x + pivot.x, rotated.y + pivot.y)
        }

        fn access() -> MutexGuard<'static, Scene> {
            static GLOBSTATE: Mutex<Scene> = Mutex::new(Scene {
                camera: Camera::new(),
                light: Camera::new(),
                mouse: Mouse::new(),
                command_input: CommandQueue::new(),
                selection_cube: Cube::new(),
                grid_xz: Grid::new(),
                model: Model::new(),
                selection_position: [0, 0, 0],
                selection_radius: 1,
                selection_shape: SelectionShape::Sphere,
                material_color: [0.8, 0.8, 0.8, 1.0],
                initialized: false,
            });
            GLOBSTATE.lock().unwrap()
        }

        pub fn queue_command(command: Command) {
            let mut scene = Self::access();

            scene.command_input.queue_command(command);
        }

        pub fn set_scene_name(name: String) {
            let mut scene = Self::access();

            scene.set_name(name);
        }

        pub fn set_name(&mut self, name: String) {
            self.model.set_name(name);
        }

        pub fn handle_mouse_down(scene: &mut Scene) {
            scene.mouse.is_pressed = true;
        }

        pub fn handle_mouse_up(scene: &mut Scene) {
            scene.mouse.is_pressed = false;
        }

        pub fn handle_mouse_moved(command: &Command, scene: &mut Scene) {
            let current_position = Point2::new(command.data1 as i32, command.data2 as i32);

            if scene.mouse.is_pressed {
                let position_diff = Point2::new(
                    current_position.x - scene.mouse.last_position.x,
                    current_position.y - scene.mouse.last_position.y,
                );
                let current_camera_eye = scene.camera.eye;
                let current_camera_target = scene.camera.target;
                let blunting = 100.0;
                let current_camera_eye_2d = Point2::new(current_camera_eye.x, current_camera_eye.z);
                let current_camera_target_2d =
                    Point2::new(current_camera_target.x, current_camera_target.z);
                // rotate the eye around the target
                let adjusted = Self::rotate_2d(
                    current_camera_eye_2d,
                    current_camera_target_2d,
                    position_diff.x as f32 / blunting,
                );

                scene.camera.eye = Point3::new(adjusted.x, current_camera_eye.y, adjusted.y);

                // now do the same thing for vertical axis
                let current_camera_eye = scene.camera.eye;
                let current_camera_eye_2d = Point2::new(current_camera_eye.y, current_camera_eye.z);
                let current_camera_target_2d =
                    Point2::new(current_camera_target.y, current_camera_target.z);
                // rotate the eye around the target
                let adjusted = Self::rotate_2d(
                    current_camera_eye_2d,
                    current_camera_target_2d,
                    -position_diff.y as f32 / blunting,
                );

                scene.camera.eye = Point3::new(current_camera_eye.x, adjusted.x, adjusted.y);
            }
            scene.mouse.last_position = current_position;
        }

        pub fn handle_move_up(scene: &mut Scene) {
            scene.camera.eye = Point3::new(
                scene.camera.eye.x,
                scene.camera.eye.y + 0.1 as f32,
                scene.camera.eye.z,
            );
            scene.camera.target = Point3::new(
                scene.camera.target.x,
                scene.camera.target.y + 0.1 as f32,
                scene.camera.target.z,
            );
        }

        pub fn handle_move_down(scene: &mut Scene) {
            scene.camera.eye = Point3::new(
                scene.camera.eye.x,
                scene.camera.eye.y - 0.1 as f32,
                scene.camera.eye.z,
            );
            scene.camera.target = Point3::new(
                scene.camera.target.x,
                scene.camera.target.y - 0.1 as f32,
                scene.camera.target.z,
            );
        }

        pub fn handle_move_left(scene: &mut Scene) {
            let diff = scene.camera.target - scene.camera.eye;
            let blunting = 10.0;
            //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
            let projection = Vector3::new(diff.z, 0.0, -diff.x) / blunting;

            scene.camera.eye += projection;
            scene.camera.target += projection;
        }

        pub fn handle_move_right(scene: &mut Scene) {
            let diff = scene.camera.target - scene.camera.eye;
            let blunting = 10.0;
            //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
            let projection = Vector3::new(diff.z, 0.0, -diff.x) / blunting;

            scene.camera.eye -= projection;
            scene.camera.target -= projection;
        }

        pub fn handle_move_forward(scene: &mut Scene) {
            let diff = scene.camera.target - scene.camera.eye;
            let blunting = 10.0;
            let projection = Vector3::new(diff.x, 0.0, diff.z) / blunting;

            scene.camera.eye += projection;
            scene.camera.target += projection;
        }

        pub fn handle_move_backward(scene: &mut Scene) {
            let diff = scene.camera.target - scene.camera.eye;
            let blunting = 10.0;
            let projection = Vector3::new(-diff.x, 0.0, -diff.z) / blunting;

            scene.camera.eye += projection;
            scene.camera.target += projection;
        }

        pub fn handle_toggle_voxel(scene: &mut Scene) {
            let selections = Self::selection_voxels(
                &scene.selection_position,
                scene.selection_radius as i32,
                scene.selection_shape,
            );

            let value: bool = scene.model.all_voxels_active(&selections);

            if value {
                log::info!("Toggle all voxels active: TRUE");
            } else {
                log::info!("Toggle all voxels active: FALSE");
            }
            for selection in selections {
                scene
                    .model
                    .toggle_voxel(selection, !value, scene.material_color);
            }
        }

        pub async fn save_scene() {
            let scene = Self::access();
            scene.model.save().await;
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

        pub fn handle_toggle_selection_shape(scene: &mut Scene) {
            scene.selection_shape = if scene.selection_shape == SelectionShape::Sphere {
                SelectionShape::Cube
            } else {
                SelectionShape::Sphere
            }
        }

        pub fn handle_mouse_scroll(command: &Command, scene: &mut Scene) {
            let direction: u32 = command.data1;
            let max_selection_radius: u32 = 16;
            let min_selection_radius: u32 = 1;

            if direction > 0 {
                scene.selection_radius = min(scene.selection_radius + 1, max_selection_radius);
            } else {
                scene.selection_radius = max(scene.selection_radius - 1, min_selection_radius);
            }
        }

        pub fn handle_key_down(command: &Command, scene: &mut Scene) {
            let key = command.data1;

            match key {
                // E
                69 => Self::handle_move_up(scene),
                // C
                67 => Self::handle_move_down(scene),
                // A or LEFT
                65 | 37 => Self::handle_move_left(scene),
                // D or RIGHT
                68 | 39 => Self::handle_move_right(scene),
                // W or UP
                87 | 38 => Self::handle_move_forward(scene),
                // S or X or DOWN
                83 | 88 | 40 => Self::handle_move_backward(scene),
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
                // T
                84 => Self::handle_toggle_selection_shape(scene),
                _ => log::info!("Unhandled key press: {}", key),
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
                    None => break,
                }
            }
        }

        pub fn init_scene(canvas_id: &str) {
            let mut scene = Self::access();
            scene.init(canvas_id);
        }

        pub fn set_scene_material_color(red_str: &str, green_str: &str, blue_str: &str) {
            let mut scene = Self::access();
            scene.set_material_color(red_str, green_str, blue_str);
        }

        pub fn set_material_color(&mut self, red_str: &str, green_str: &str, blue_str: &str) {
            let red = red_str.parse::<i32>().unwrap();
            let red_f32 = red as f32 / 255.0;
            let green = green_str.parse::<i32>().unwrap();
            let green_f32 = green as f32 / 255.0;
            let blue = blue_str.parse::<i32>().unwrap();
            let blue_f32 = blue as f32 / 255.0;

            self.material_color = [red_f32, green_f32, blue_f32, 1.0];
        }

        pub async fn load_scene() {
            let mut scene = Self::access();
            scene.initialized = false;
            scene.model.load_scene().await;

            scene.initialized = true;
        }

        pub async fn delete_scene() {
            let mut scene = Self::access();
            scene.model.delete_scene().await;
        }

        pub async fn load_first_scene() {
            let mut scene = Self::access();
            scene.initialized = false;
            scene.model.load_first_scene().await;
            scene.initialized = true;
        }

        pub fn init(&mut self, canvas_id: &str) {
            self.light.eye = Point3::new(15.0, 60.0, 14.0);
            self.light.target = Point3::new(0.0, 0.0, 0.0);
            //self.light.eye = Point3::new(8.0, 6.0, 20.0);
            //self.light.target = Point3::new(1.0, 0.0, 10.0);
            self.selection_cube.init();
            self.grid_xz.init();
            self.grid_xz.rotate([(90.0 as f32).to_radians(), 0.0, 0.0]);

            self.model.init();

            let document = web_sys::window().unwrap().document().unwrap();
            let canvas_element = document.get_element_by_id(canvas_id).unwrap();
            let canvas: web_sys::HtmlCanvasElement =
                match canvas_element.dyn_into::<web_sys::HtmlCanvasElement>() {
                    Ok(canvas) => canvas,
                    Err(_) => {
                        panic!("Could not find the canvas element");
                    }
                };

            let key_down_closure = EventListener::new(&canvas, "keydown", move |event| {
                let key_event = event.clone().dyn_into::<web_sys::KeyboardEvent>().unwrap();
                log::info!("Key down");
                Scene::queue_command(Command {
                    command_type: CommandType::KeyDown,
                    data1: key_event.key_code() as u32,
                    data2: key_event.key_code() as u32,
                });
            });

            key_down_closure.forget();

            let mouse_move_closure = EventListener::new(&canvas, "mousemove", move |event| {
                let move_event = event.clone().dyn_into::<web_sys::MouseEvent>().unwrap();

                // The contents of the closure are only run when the
                // closure is called by the JS event handler.
                // The code inside the closures is the only part of this
                // program that runs repeatedly.

                Scene::queue_command(Command {
                    command_type: CommandType::MouseMoved,
                    data1: move_event.offset_x() as u32,
                    data2: move_event.offset_y() as u32,
                });
            });

            mouse_move_closure.forget();

            let wheel_closure = EventListener::new(&canvas, "wheel", move |event| {
                let wheel_event = event.clone().dyn_into::<web_sys::WheelEvent>().unwrap();

                let direction = if wheel_event.delta_y() < 0.0 {
                    1 as u32
                } else {
                    0 as u32
                };
                Scene::queue_command(Command {
                    command_type: CommandType::MouseScroll,
                    data1: direction,
                    data2: 1,
                });
            });

            wheel_closure.forget();

            let mouse_down_closure = EventListener::new(&canvas, "mousedown", move |_event| {
                Scene::queue_command(Command {
                    command_type: CommandType::MouseDown,
                    data1: 1,
                    data2: 1,
                });
            });

            mouse_down_closure.forget();

            let mouse_up_closure = EventListener::new(&canvas, "mouseup", move |_event| {
                Scene::queue_command(Command {
                    command_type: CommandType::MouseUp,
                    data1: 1,
                    data2: 1,
                });
            });

            mouse_up_closure.forget();
            self.initialized = true;
        }

        pub fn calculate_distance_squared(from: &[i32; 3], to: &[i32; 3]) -> i32 {
            (from[0] - to[0]).pow(2) + (from[1] - to[1]).pow(2) + (from[2] - to[2]).pow(2)
        }

        pub fn selection_voxels(
            center: &[i32; 3],
            radius: i32,
            shape: SelectionShape,
        ) -> Vec<[i32; 3]> {
            let mut voxels = Vec::new();
            let range: i32 = 16;
            let radius_squared: i32 = radius.pow(2);

            if shape == SelectionShape::Sphere {
                for x in -range..range {
                    for y in -range..range {
                        for z in -range..range {
                            let voxel_position = [x, y, z];
                            let distance: i32 =
                                Self::calculate_distance_squared(center, &voxel_position);

                            if distance < radius_squared {
                                voxels.push([x, y, z]);
                            }
                        }
                    }
                }
            } else {
                for x in -range..range {
                    for y in -range..range {
                        for z in -range..range {
                            let voxel_position = [x, y, z];
                            if (center[0] - voxel_position[0]).abs() < radius
                                && (center[1] - voxel_position[1]).abs() < radius
                                && (center[2] - voxel_position[2]).abs() < radius
                            {
                                voxels.push([x, y, z]);
                            }
                        }
                    }
                }
            }

            voxels
        }

        pub fn draw(graphics: &Graphics) {
            let mut scene = Self::access();
            if !scene.initialized {
                return;
            }

            graphics.prepare_shadow_frame();

            let light = if !graphics.swap_cameras {
                scene.light
            } else {
                scene.camera
            };
            let camera = if !graphics.swap_cameras {
                scene.camera
            } else {
                scene.light
            };

            if !graphics.swap_shaders {
                for voxel in scene.model.drawables().iter() {
                    graphics.draw_shadow(voxel, WebGlRenderingContext::TRIANGLES, light);
                }
            }

            graphics.finish_shadow_frame();

            graphics.prepare_camera_frame();

            let selections = Self::selection_voxels(
                &scene.selection_position,
                scene.selection_radius as i32,
                scene.selection_shape,
            );

            for selection in selections {
                scene.selection_cube.translation = [
                    selection[0] as f32,
                    selection[1] as f32,
                    selection[2] as f32,
                ];
                graphics.draw(
                    &scene.selection_cube,
                    WebGlRenderingContext::TRIANGLES,
                    camera,
                    light,
                );
            }
            graphics.draw(&scene.grid_xz, WebGlRenderingContext::LINES, camera, light);

            for voxel in scene.model.drawables().iter() {
                graphics.draw(voxel, WebGlRenderingContext::TRIANGLES, camera, light);
            }

            graphics.finish_camera_frame();
        }
    }
}
