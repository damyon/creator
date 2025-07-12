use crate::{cube::Cube, drawable::Drawable};
use serde::{Deserialize, Serialize};

/// Helper function to create an empty list.
/// The scope is odd.
fn empty_list() -> [Option<Box<Ocnode>>; 8] {
    [None, None, None, None, None, None, None, None]
}

pub const LEVELS: u32 = 7;

/// A struct representing a single cube for the octree.
/// Cubes contain children which are smaller cubes.
#[derive(Serialize, Deserialize, Clone)]
pub struct Ocnode {
    /// the x index of the cube.
    #[serde(rename = "x")]
    x_index: i32,
    /// the y index of the cube.
    #[serde(rename = "y")]
    y_index: i32,
    /// the z index of the cube.
    #[serde(rename = "z")]
    z_index: i32,
    /// How many parents does this cube have.
    #[serde(rename = "level")]
    sub_division_level: u32,
    /// Is this cube empty or filled?
    active: bool,
    /// We don't serialize this directly but this is the smaller cubes inside this one.
    #[serde(skip)]
    #[serde(default = "empty_list")]
    children: [Option<Box<Self>>; 8],
    /// Does this cube contain smaller ones?
    has_children: bool,
    /// The color of the cube including alpha channel.
    color: [f32; 4],
    /// Render this node with fluid animation
    fluid: i32,
}

impl Ocnode {
    /// Create a new empty cube.
    pub const fn new() -> Ocnode {
        Ocnode {
            x_index: -Ocnode::range(),
            y_index: -Ocnode::range(),
            z_index: -Ocnode::range(),
            sub_division_level: 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: [0.8, 0.8, 0.8, 0.8],
            fluid: 0,
        }
    }

    /// Return the coordinate range. The actual positions go from -range to +range
    pub const fn range() -> i32 {
        2i32.pow(LEVELS - 1) / 2
    }

    /// Calculate the width of a cube at this subdivision level
    pub fn resolution(&self, sub_division_level: u32) -> u32 {
        let power = LEVELS.checked_sub(sub_division_level).expect("");
        2u32.pow(power)
    }

    /// Get the list of active cubes including this one and all it's children.
    pub fn active_nodes(&self) -> Vec<Ocnode> {
        let mut found: Vec<Ocnode> = vec![];

        if self.active {
            found.push(self.clone());
        }
        if self.has_children {
            let squirts = self.children.each_ref();

            for node_opt in squirts {
                match node_opt {
                    None => {
                        log::debug!("Should not get here")
                    }
                    Some(node) => {
                        found.extend(node.active_nodes());
                    }
                };
            }
        }

        found
    }

    /// Set this cube and all it's children to hidden.
    pub fn clear(&mut self) {
        self.active = false;

        let squirts = self.children.each_mut();

        for node_opt in squirts {
            match node_opt {
                None => {}
                Some(squirt) => {
                    squirt.clear();
                }
            };
        }
    }

    /// Used when restoring from serial form.
    pub fn apply(&mut self, node: &Ocnode) {
        if node.x_index == self.x_index
            && node.y_index == self.y_index
            && node.z_index == self.z_index
            && node.sub_division_level == self.sub_division_level
        {
            // We got a match. Apply it.
            self.active = node.active;
            self.color = node.color;
            self.fluid = node.fluid;
        }
        let squirts = self.children.each_mut();

        for node_opt in squirts {
            match node_opt {
                None => {}
                Some(squirt) => {
                    squirt.apply(node);
                }
            };
        }
    }

    /// Determine the distance between this cube and the camera.
    fn depth(&self, camera: [f32; 3]) -> f32 {
        ((self.x_index as f32 - camera[0]).powi(2)
            + (self.y_index as f32 - camera[1]).powi(2)
            + (self.z_index as f32 - camera[2]).powi(2))
        .sqrt()
    }

    /// Set the active state to match the combined active state of all children.
    pub fn optimize(&mut self, camera_eye: [f32; 3]) {
        if self.has_children {
            // Optimize leaf first then move up the tree.
            let squirts = self.children.each_mut();
            for child in squirts {
                match child {
                    None => {}
                    Some(down) => {
                        down.optimize(camera_eye);
                    }
                }
            }
            let squirts = self.children.each_mut();
            let has_peg = squirts
                .into_iter()
                .any(|child| child.as_ref().expect("child").active);

            let squirts = self.children.each_mut();
            let has_hole = squirts
                .into_iter()
                .any(|child| !child.as_ref().expect("child").active);
            let squirts = self.children.each_mut();
            let mut color = [0.0, 0.0, 0.0, 0.0];
            let mut fluid = 0;
            match squirts[0] {
                None => {
                    log::debug!("Should not get here")
                }
                Some(node) => {
                    color = node.color;
                    fluid = node.fluid;
                }
            };
            let squirts = self.children.each_mut();
            let not_uniform_color = squirts.into_iter().any(|child| {
                let compare = child.as_ref().expect("child").color;
                let compare_fluid = child.as_ref().expect("child").fluid;
                compare[0] != color[0]
                    || compare[1] != color[1]
                    || compare[2] != color[2]
                    || compare[3] != color[3]
                    || compare_fluid != fluid
            });

            let res = LEVELS.checked_sub(self.sub_division_level).expect("");
            let depth = self.depth(camera_eye) / res as f32;
            let lod = 60.0;

            self.active = (has_peg && (depth > lod)) || (!has_hole && !not_uniform_color);
            let first = self.children.first();

            match first {
                None => {}
                Some(node) => {
                    self.color = node.as_ref().unwrap().color;
                    self.fluid = node.as_ref().unwrap().fluid;
                }
            }
        }
    }

    /// Are all the nodes in the list of nodes active?
    pub fn all_voxels_active(&self, positions: &Vec<[i32; 3]>) -> bool {
        for position in positions {
            if self.x_index == position[0]
                && self.y_index == position[1]
                && self.z_index == position[2]
                && self.sub_division_level == LEVELS
                && !self.active
            {
                return false;
            }
        }
        let squirts = self.children.each_ref();

        for node_opt in squirts {
            match node_opt {
                None => {}
                Some(node) => {
                    if !node.all_voxels_active(positions) {
                        return false;
                    }
                }
            };
        }

        true
    }

    /// Search this node and it's children and switch the toggle state if the posistion is correct.
    pub fn toggle_voxel(&mut self, position: [i32; 3], value: bool, color: [f32; 4], fluid: i32) {
        if self.x_index == position[0]
            && self.y_index == position[1]
            && self.z_index == position[2]
            && self.sub_division_level == LEVELS
        {
            self.active = value;
            self.color = color;
            self.fluid = fluid;
        }
        let squirts = self.children.each_mut();

        for node_opt in squirts {
            match node_opt {
                None => {}
                Some(node) => {
                    node.toggle_voxel(position, value, color, fluid);
                }
            };
        }
    }

    /// Generate a list of drawables from the active cubes in this one.
    pub fn drawables(&mut self) -> Vec<Cube> {
        if self.has_children {
            if self.active {
                let scale = self.resolution(self.sub_division_level) as f32;
                let mut cube = Cube::new();

                cube.color = self.color;
                cube.fluid = self.fluid;
                cube.scale = scale;
                cube.init();

                let x = self.x_index as f32 * (1.0);
                let y = self.y_index as f32 * (1.0);
                let z = self.z_index as f32 * (1.0);
                cube.translate([x, y, z]);

                vec![cube]
            } else {
                let mut child_cubes: Vec<Cube> = vec![];
                let squirts = self.children.each_mut();

                for node_opt in squirts {
                    match node_opt {
                        None => {}
                        Some(node) => {
                            let mut cube = node.drawables();

                            child_cubes.append(&mut cube);
                        }
                    };
                }
                child_cubes
            }
        } else if self.active {
            let scale = 1.0;
            let mut cube = Cube::new();

            cube.color = self.color;
            cube.fluid = self.fluid;
            cube.scale = scale;
            cube.init();

            let x = self.x_index as f32 * (scale);
            let y = self.y_index as f32 * (scale);
            let z = self.z_index as f32 * (scale);

            cube.translate([x, y, z]);

            vec![cube]
        } else {
            vec![]
        }
    }

    /// Create smaller children cubes from this outer cube.
    pub fn decimate(&mut self, sub_division_level: u32) {
        if sub_division_level - 1 > 0 {
            self.subdivide();

            let squirts = self.children.each_mut();

            for node_opt in squirts {
                match node_opt {
                    None => {
                        log::debug!("Should not get here")
                    }
                    Some(node) => {
                        node.decimate(sub_division_level - 1);
                    }
                };
            }
        }
    }

    /// Used by the decimate function to create smaller cubes.
    pub fn subdivide(&mut self) {
        self.has_children = true;

        self.children[0] = Some(Box::new(Ocnode {
            x_index: self.x_index,
            y_index: self.y_index,
            z_index: self.z_index,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
        }));

        self.children[1] = Some(Box::new(Ocnode {
            x_index: self.x_index + self.resolution(self.sub_division_level + 1) as i32,
            y_index: self.y_index,
            z_index: self.z_index,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
        }));
        self.children[2] = Some(Box::new(Ocnode {
            x_index: self.x_index,
            y_index: self.y_index + self.resolution(self.sub_division_level + 1) as i32,
            z_index: self.z_index,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
        }));
        self.children[3] = Some(Box::new(Ocnode {
            x_index: self.x_index,
            y_index: self.y_index,
            z_index: self.z_index + self.resolution(self.sub_division_level + 1) as i32,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
        }));
        self.children[4] = Some(Box::new(Ocnode {
            x_index: self.x_index + self.resolution(self.sub_division_level + 1) as i32,
            y_index: self.y_index + self.resolution(self.sub_division_level + 1) as i32,
            z_index: self.z_index,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
        }));
        self.children[5] = Some(Box::new(Ocnode {
            x_index: self.x_index,
            y_index: self.y_index + self.resolution(self.sub_division_level + 1) as i32,
            z_index: self.z_index + self.resolution(self.sub_division_level + 1) as i32,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
        }));
        self.children[6] = Some(Box::new(Ocnode {
            x_index: self.x_index + self.resolution(self.sub_division_level + 1) as i32,
            y_index: self.y_index,
            z_index: self.z_index + self.resolution(self.sub_division_level + 1) as i32,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
        }));
        self.children[7] = Some(Box::new(Ocnode {
            x_index: self.x_index + self.resolution(self.sub_division_level + 1) as i32,
            y_index: self.y_index + self.resolution(self.sub_division_level + 1) as i32,
            z_index: self.z_index + self.resolution(self.sub_division_level + 1) as i32,
            sub_division_level: self.sub_division_level + 1,
            active: false,
            children: [None, None, None, None, None, None, None, None],
            has_children: false,
            color: self.color,
            fluid: self.fluid,
        }));
    }
}
