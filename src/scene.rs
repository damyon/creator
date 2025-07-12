use js_sys::Math::random;
use std::cmp::{max, min};
use std::sync::{Mutex, MutexGuard};
use web_sys::WebGlRenderingContext;

use crate::command::{Command, CommandType};
use crate::command_queue::CommandQueue;
use crate::drawable::Drawable;
use crate::graphics::Graphics;
use crate::grid::Grid;
use crate::model::Model;
use crate::mouse::Mouse;
use crate::ocnode::Ocnode;
use crate::storage::Storage;
use crate::stored_octree::StoredOctree;
use crate::{camera::Camera, cube::Cube};
use gloo::events::EventListener;
use wasm_bindgen::JsCast;

extern crate nalgebra as na;
extern crate nalgebra_glm as glm;

use na::{Point2, Point3, Vector3};

/// Simple list of supported selection shapes.
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SelectionShape {
    Sphere,
    Cube,
    SquareXZ,
    SquareXY,
    SquareYZ,
    CircleXZ,
    CircleXY,
    CircleYZ,
}

/// This represents the data and the links to input/output required to render the scene.
pub struct Scene {
    /// The current camera.
    pub camera: Camera,
    /// The current light.
    pub light: Camera,
    /// The mouse info.
    mouse: Mouse,
    /// A queue of commands waiting to be processed.
    command_input: CommandQueue,
    /// A cube that is used to draw the selection shape.
    selection_cube: Cube,
    /// We could show more, but only the flat grid is enough.
    grid_xz: Grid,
    /// This is the octree of voxels.
    model: Model,
    /// Where is the selection.
    selection_position: [i32; 3],
    /// What is the size of the selection.
    selection_radius: u32,
    /// What shape is the selection.
    selection_shape: SelectionShape,
    /// What colour will we fill if the selection is toggled.
    material_color: [f32; 4],
    /// Are we currently drawing a frame?
    drawing: bool,
    /// Should we skip the next frame?
    throttle: u32,
    /// Are we loading from browser?
    loading: bool,
    /// Is the color smooth?
    smooth: bool,
    /// Is the material fluid?
    fluid: i32,
    /// Will the frame match the last rendered frame?
    dirty: bool,
    /// Approximation of time
    elapsed: f32,
}

impl Scene {
    /// Helper function to rotate a point around an axis.
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

    /// Used to lock/release a global scene ref.
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
            drawing: false,
            throttle: 10,
            loading: true,
            smooth: true,
            fluid: 0,
            dirty: true,
            elapsed: 0.0,
        });
        GLOBSTATE.lock().unwrap()
    }

    /// Add a command to the queue of commands to process later.
    pub fn queue_command(command: Command) {
        let mut scene = Self::access();
        scene.dirty = true;

        scene.command_input.queue_command(command);
    }

    /// Change the global scene name.
    pub fn set_scene_name(name: String) {
        let mut scene = Self::access();

        scene.set_name(name);
    }

    /// Change this scene name.
    pub fn set_name(&mut self, name: String) {
        self.model.set_name(name);
    }

    /// Process a mouse down event.
    pub fn handle_mouse_down(scene: &mut Scene) {
        scene.mouse.is_pressed = true;
    }

    /// Process a mouse up event.
    pub fn handle_mouse_up(scene: &mut Scene) {
        scene.mouse.is_pressed = false;
    }

    /// Process a mouse moved event.
    pub fn handle_mouse_moved(command: &Command, scene: &mut Scene) {
        let current_position = Point2::new(command.data1 as i32, command.data2 as i32);

        if scene.mouse.is_pressed {
            let position_diff = Point2::new(
                current_position.x - scene.mouse.last_position.x,
                current_position.y - scene.mouse.last_position.y,
            );
            let current_camera_eye = scene.camera.eye;
            let current_camera_target = scene.camera.target;
            let current_camera_direction = current_camera_target - current_camera_eye;

            let current_camera_distance = (current_camera_direction.x.powf(2.0f32)
                + current_camera_direction.z.powf(2.0f32))
            .sqrt();
            let scale = 20.0f32 / current_camera_distance;
            let scaled_direction = scale * current_camera_direction;
            let scaled_point =
                Point3::new(scaled_direction.x, scaled_direction.y, scaled_direction.z);
            let blunting = 100.0;
            let current_camera_eye_2d = Point2::new(current_camera_eye.x, current_camera_eye.z);
            let current_camera_target_2d = Point2::new(
                scaled_point.x + current_camera_eye.x,
                scaled_point.z + current_camera_eye.z,
            );
            // rotate the eye around the target
            let adjusted = Self::rotate_2d(
                current_camera_eye_2d,
                current_camera_target_2d,
                position_diff.x as f32 / blunting,
            );

            scene.camera.eye = Point3::new(adjusted.x, current_camera_eye.y, adjusted.y);

            // Up down does not need rotation.

            scene.camera.eye.y += position_diff.y as f32 / 10.0f32;

            let camera_eye = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];
            scene.model.optimize(camera_eye);
        }
        scene.mouse.last_position = current_position;
    }

    /// The key was pressed to move up.
    pub fn handle_move_up(scene: &mut Scene) {
        scene.camera.eye = Point3::new(
            scene.camera.eye.x,
            scene.camera.eye.y + 0.1_f32,
            scene.camera.eye.z,
        );
        scene.camera.target = Point3::new(
            scene.camera.target.x,
            scene.camera.target.y + 0.1_f32,
            scene.camera.target.z,
        );

        let camera_eye = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];
        scene.model.optimize(camera_eye);
    }

    /// The key was pressed to move down.
    pub fn handle_move_down(scene: &mut Scene) {
        scene.camera.eye = Point3::new(
            scene.camera.eye.x,
            scene.camera.eye.y - 0.1_f32,
            scene.camera.eye.z,
        );
        scene.camera.target = Point3::new(
            scene.camera.target.x,
            scene.camera.target.y - 0.1_f32,
            scene.camera.target.z,
        );
        let camera_eye = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];
        scene.model.optimize(camera_eye);
    }

    /// The key was pressed to move left.
    pub fn handle_move_left(scene: &mut Scene) {
        let diff = scene.camera.target - scene.camera.eye;
        let blunting = 10.0;
        //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
        let projection = Vector3::new(diff.z, 0.0, -diff.x) / blunting;

        scene.camera.eye += projection;
        scene.camera.target += projection;
        let camera_eye = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];
        scene.model.optimize(camera_eye);
    }

    /// The key was pressed to move right.
    pub fn handle_move_right(scene: &mut Scene) {
        let diff = scene.camera.target - scene.camera.eye;
        let blunting = 10.0;
        //To rotate a vector 90 degrees clockwise, you can change the coordinates from (x,y) to (y,−x).
        let projection = Vector3::new(diff.z, 0.0, -diff.x) / blunting;

        scene.camera.eye -= projection;
        scene.camera.target -= projection;
        let camera_eye = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];
        scene.model.optimize(camera_eye);
    }

    /// The key was pressed to move forward.
    pub fn handle_move_forward(scene: &mut Scene) {
        let diff = scene.camera.target - scene.camera.eye;
        let blunting = 10.0;
        let projection = Vector3::new(diff.x, 0.0, diff.z) / blunting;

        scene.camera.eye += projection;
        scene.camera.target += projection;
        let camera_eye = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];
        scene.model.optimize(camera_eye);
    }

    /// The key was pressed to move backwards.
    pub fn handle_move_backward(scene: &mut Scene) {
        let diff = scene.camera.target - scene.camera.eye;
        let blunting = 10.0;
        let projection = Vector3::new(-diff.x, 0.0, -diff.z) / blunting;

        scene.camera.eye += projection;
        scene.camera.target += projection;
        let camera_eye = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];
        scene.model.optimize(camera_eye);
    }

    /// The key was pressed to toggle the state of the current selection.
    pub fn handle_toggle_voxel(scene: &mut Scene) {
        let selections = Self::selection_voxels(
            &scene.selection_position,
            scene.selection_radius as i32,
            scene.selection_shape,
        );

        let value: bool = scene.model.all_voxels_active(&selections);
        let count = selections.len();
        let fluid = scene.fluid;
        if value {
            log::info!("Toggle all voxels active: TRUE {count} {fluid}");
        } else {
            log::info!("Toggle all voxels active: FALSE {count} {fluid}");
        }
        for selection in selections {
            log::info!("Selection {:?}", selection);
            let bump = if scene.smooth {
                0.0f32
            } else {
                random() as f32 / 10.0 - 0.05
            };
            let color = [
                (scene.material_color[0] + bump).clamp(0.0, 1.0),
                (scene.material_color[1] + bump).clamp(0.0, 1.0),
                (scene.material_color[2] + bump).clamp(0.0, 1.0),
                (scene.material_color[3]).clamp(0.0, 1.0),
            ];
            let camera_eye = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];
            scene
                .model
                .toggle_voxel(selection, !value, color, camera_eye, scene.fluid);
        }
    }

    /// Save the scene to the browser.
    pub async fn save_scene() {
        // The point of this scope shananigens is the model save operation is slow
        // and doesn't need access to anything from scope outside of the model.
        let model: Model = {
            let scene = Self::access();
            scene.model.clone()
        };
        model.save().await;
    }

    /// Move the selection shape left.
    pub fn handle_move_selection_left(scene: &mut Scene) {
        scene.selection_cube.translate([-1.0, 0.0, 0.0]);
        scene.selection_position[0] -= 1;
    }

    /// Move the selection shape right.
    pub fn handle_move_selection_right(scene: &mut Scene) {
        scene.selection_cube.translate([1.0, 0.0, 0.0]);
        scene.selection_position[0] += 1;
    }

    /// Move the selection shape forward.
    pub fn handle_move_selection_forward(scene: &mut Scene) {
        scene.selection_cube.translate([0.0, 0.0, 1.0]);
        scene.selection_position[2] += 1;
    }

    /// Move the selection shape backward.
    pub fn handle_move_selection_backward(scene: &mut Scene) {
        scene.selection_cube.translate([0.0, 0.0, -1.0]);
        scene.selection_position[2] -= 1;
    }

    /// Move the selection shape up.
    pub fn handle_move_selection_up(scene: &mut Scene) {
        scene.selection_cube.translate([0.0, 1.0, 0.0]);
        scene.selection_position[1] += 1;
    }

    /// Move the selection shape down.
    pub fn handle_move_selection_down(scene: &mut Scene) {
        scene.selection_cube.translate([0.0, -1.0, 0.0]);
        scene.selection_position[1] -= 1;
    }

    /// Hide or show the selection shape for the global scene.
    pub fn scene_toggle_selection_shape() {
        let mut scene = Self::access();

        Self::handle_toggle_selection_shape(&mut scene);
    }

    /// Hide or show the selection shape.
    pub fn handle_toggle_selection_shape(scene: &mut Scene) {
        scene.selection_shape = if scene.selection_shape == SelectionShape::Sphere {
            SelectionShape::Cube
        } else if scene.selection_shape == SelectionShape::Cube {
            SelectionShape::SquareXZ
        } else if scene.selection_shape == SelectionShape::SquareXZ {
            SelectionShape::SquareXY
        } else if scene.selection_shape == SelectionShape::SquareXY {
            SelectionShape::SquareYZ
        } else if scene.selection_shape == SelectionShape::SquareYZ {
            SelectionShape::CircleXZ
        } else if scene.selection_shape == SelectionShape::CircleXZ {
            SelectionShape::CircleXY
        } else if scene.selection_shape == SelectionShape::CircleXY {
            SelectionShape::CircleYZ
        } else {
            SelectionShape::Sphere
        }
    }

    /// Handle the mouse scroll.
    pub fn handle_mouse_scroll(command: &Command, scene: &mut Scene) {
        let direction: u32 = command.data1;
        let max_selection_radius: u32 = 32;
        let min_selection_radius: u32 = 1;

        if direction > 0 {
            scene.selection_radius = min(scene.selection_radius + 1, max_selection_radius);
        } else {
            scene.selection_radius = max(scene.selection_radius - 1, min_selection_radius);
        }
    }

    /// Handle a key press.
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
            //84 => Self::handle_toggle_selection_shape(scene),
            _ => log::info!("Unhandled key press: {}", key),
        }
    }

    /// Process the command queue.
    pub fn process_commands() {
        let mut scene = Self::access();

        let mut command_opt = scene.command_input.next();

        while let Some(command) = command_opt {
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
    }

    /// Create a new scene.
    pub fn init_scene() {
        let mut scene = Self::access();
        scene.init();
    }

    /// Should we render the current frame?
    pub fn throttle() -> bool {
        let mut scene = Self::access();

        if !scene.dirty {
            return true;
        }

        if scene.loading {
            return true;
        }

        if !scene.drawing {
            return true;
        }

        scene.throttle -= 1;
        if scene.throttle >= 1 {
            return true;
        }
        scene.throttle = 2;
        false
    }

    /// Change the active color.
    pub fn set_scene_material_color(
        red_str: &str,
        green_str: &str,
        blue_str: &str,
        alpha_str: &str,
    ) {
        let mut scene = Self::access();
        scene.set_material_color(red_str, green_str, blue_str, alpha_str);
    }

    /// Change the active color for this scene.
    pub fn set_material_color(
        &mut self,
        red_str: &str,
        green_str: &str,
        blue_str: &str,
        alpha_str: &str,
    ) {
        log::debug!("Set material color ({red_str}, {green_str}, {blue_str}, {alpha_str})");
        let red = red_str.parse::<i32>().unwrap();
        let red_f32 = red as f32 / 255.0;
        let green = green_str.parse::<i32>().unwrap();
        let green_f32 = green as f32 / 255.0;
        let blue = blue_str.parse::<i32>().unwrap();
        let blue_f32 = blue as f32 / 255.0;
        let alpha_f32 = alpha_str.parse::<f32>().unwrap();

        self.material_color = [red_f32, green_f32, blue_f32, alpha_f32];
        self.selection_cube.color = [red_f32, green_f32, blue_f32, 0.5];
    }

    /// Load a scene from the browser.
    pub async fn load_scene() {
        let name = {
            let mut scene = Self::access();
            scene.drawing = false;
            scene.loading = true;
            scene.model.voxels.name.clone()
        };

        let storage = Storage::new();
        let serial: Option<StoredOctree> = storage.load_scene(name).await;
        if serial.is_some() {
            let mut scene = Self::access();
            let camera_eye = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];
            scene
                .model
                .voxels
                .load_from_serial(serial.unwrap(), camera_eye);
            scene.drawing = true;
            scene.loading = false;
        }
    }

    /// Delete a scene from the browser.
    pub async fn delete_scene() {
        let model = {
            let mut scene = Self::access();

            scene.model.voxels.clear();
            scene.model.clone()
        };
        model.delete_scene().await;
    }

    /// Enable color noise.
    pub async fn toggle_noise() {
        let mut scene = Self::access();
        scene.smooth = false;
    }

    /// Enable smoothing.
    pub async fn toggle_smooth() {
        let mut scene = Self::access();
        scene.smooth = true;
    }

    /// Enable solid material.
    pub async fn toggle_solid() {
        let mut scene = Self::access();
        log::error!("Fluid goes off");
        scene.fluid = 0;
    }

    /// Enable fluid.
    pub async fn toggle_fluid() {
        let mut scene = Self::access();
        log::error!("Fluid goes on");
        scene.fluid = 1;
    }

    /// Load the default scene.
    pub async fn load_first_scene() {
        let storage = Storage::new();
        let serial: Option<StoredOctree> = storage.load_first_scene().await;
        if serial.is_some() {
            let mut scene = Self::access();
            let camera_eye = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];
            scene
                .model
                .voxels
                .load_from_serial(serial.unwrap(), camera_eye);
            scene.drawing = true;
            scene.loading = false;
        } else {
            let mut scene = Self::access();
            scene.drawing = true;
            scene.loading = false;
        }
    }

    /// Init the scene.
    pub fn init(&mut self) {
        self.light.eye = Point3::new(15.0, 60.0, 14.0);
        self.light.target = Point3::new(0.0, 0.0, 0.0);
        self.selection_cube.scale = 0.8f32;
        self.selection_cube.color = [0.8, 0.8, 0.8, 0.5];
        self.selection_cube.init();
        self.grid_xz.init();
        self.grid_xz.rotate([90.0_f32.to_radians(), 0.0, 0.0]);

        self.model.init();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas_element = document.get_element_by_id("scene").unwrap();
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
                data1: key_event.key_code(),
                data2: key_event.key_code(),
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
                1_u32
            } else {
                0_u32
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
    }

    /// Quicker than distance - no sqrt.
    pub fn calculate_distance_squared(from: &[i32; 3], to: &[i32; 3]) -> i32 {
        (from[0] - to[0]).pow(2) + (from[1] - to[1]).pow(2) + (from[2] - to[2]).pow(2)
    }

    /// Generate voxels based on selection.
    pub fn selection_voxels(
        center: &[i32; 3],
        radius: i32,
        shape: SelectionShape,
    ) -> Vec<[i32; 3]> {
        let mut voxels = Vec::new();
        let range: i32 = Ocnode::range() * 2;
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
        } else if shape == SelectionShape::Cube {
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
        } else if shape == SelectionShape::SquareXZ {
            // SquareXZ
            for x in -range..range {
                for z in -range..range {
                    let voxel_position = [x, center[1], z];
                    if (center[0] - voxel_position[0]).abs() < radius
                        && (center[2] - voxel_position[2]).abs() < radius
                    {
                        voxels.push([x, center[1], z]);
                    }
                }
            }
        } else if shape == SelectionShape::SquareXY {
            // SquareXY
            for x in -range..range {
                for y in -range..range {
                    let voxel_position = [x, y, center[2]];
                    if (center[0] - voxel_position[0]).abs() < radius
                        && (center[1] - voxel_position[1]).abs() < radius
                    {
                        voxels.push([x, y, center[2]]);
                    }
                }
            }
        } else if shape == SelectionShape::SquareYZ {
            // SquareYZ
            for y in -range..range {
                for z in -range..range {
                    let voxel_position = [center[0], y, z];
                    if (center[1] - voxel_position[1]).abs() < radius
                        && (center[2] - voxel_position[2]).abs() < radius
                    {
                        voxels.push([center[0], y, z]);
                    }
                }
            }
        } else if shape == SelectionShape::CircleXZ {
            // CircleXZ
            for x in -range..range {
                for z in -range..range {
                    let voxel_position = [x, center[1], z];
                    if (((center[0] - voxel_position[0]).abs() as f64).powi(2)
                        + ((center[2] - voxel_position[2]).abs() as f64).powi(2))
                    .sqrt()
                        < radius as f64
                    {
                        voxels.push([x, center[1], z]);
                    }
                }
            }
        } else if shape == SelectionShape::CircleXY {
            // CircleXY
            for x in -range..range {
                for y in -range..range {
                    let voxel_position = [x, y, center[2]];
                    if (((center[0] - voxel_position[0]).abs() as f64).powi(2)
                        + ((center[1] - voxel_position[1]).abs() as f64).powi(2))
                    .sqrt()
                        < radius as f64
                    {
                        voxels.push([x, y, center[2]]);
                    }
                }
            }
        } else if shape == SelectionShape::CircleYZ {
            // CircleYZ
            for y in -range..range {
                for z in -range..range {
                    let voxel_position = [center[0], y, z];
                    if (((center[1] - voxel_position[1]).abs() as f64).powi(2)
                        + ((center[2] - voxel_position[2]).abs() as f64).powi(2))
                    .sqrt()
                        < radius as f64
                    {
                        voxels.push([center[0], y, z]);
                    }
                }
            }
        }

        voxels
    }

    /// Draw the scene.
    pub fn draw(graphics: &Graphics) {
        let mut scene = Self::access();

        scene.elapsed += 0.01;
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
                selection[0] as f32 + 0.1,
                selection[1] as f32 + 0.1,
                selection[2] as f32 + 0.1,
            ];
            graphics.draw(
                &scene.selection_cube,
                WebGlRenderingContext::TRIANGLES,
                camera,
                light,
                scene.elapsed,
            );
        }
        graphics.draw(
            &scene.grid_xz,
            WebGlRenderingContext::LINES,
            camera,
            light,
            scene.elapsed,
        );

        let mut drawables = scene.model.drawables();

        let camera_eye = [scene.camera.eye.x, scene.camera.eye.y, scene.camera.eye.z];
        drawables.sort_by(|a, b| {
            let a_dist = a.depth(camera_eye);
            let b_dist = b.depth(camera_eye);

            b_dist.partial_cmp(&a_dist).unwrap()
        });

        for voxel in drawables.iter() {
            graphics.draw(
                voxel,
                WebGlRenderingContext::TRIANGLES,
                camera,
                light,
                scene.elapsed,
            );
        }

        graphics.finish_camera_frame();
        scene.dirty = false;
    }
}
