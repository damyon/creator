pub mod ocnode {
    use crate::{cube::cube::Cube, drawable::drawable::Drawable};
    use serde::{Deserialize, Serialize};

    fn empty_list() -> [Option<Box<Ocnode>>; 8] {
        [None, None, None, None, None, None, None, None]
    }

    pub const LEVELS: u32 = 7;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Ocnode {
        #[serde(rename = "x")]
        x_index: i32,
        #[serde(rename = "y")]
        y_index: i32,
        #[serde(rename = "z")]
        z_index: i32,
        #[serde(rename = "level")]
        sub_division_level: u32,
        active: bool,
        #[serde(skip)]
        #[serde(default = "empty_list")]
        children: [Option<Box<Self>>; 8],
        has_children: bool,
        color: [f32; 4],
    }

    impl Ocnode {
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
            }
        }

        /**
         * Return the coordinate range. The actual positions go from -range to +range
         */
        pub const fn range() -> i32 {
            2i32.pow(LEVELS - 1) / 2
        }

        /**
         * Calculate the width of a cube at this subdivision level
         */
        pub fn resolution(&self, sub_division_level: u32) -> u32 {
            let power = LEVELS.checked_sub(sub_division_level).expect("");
            2u32.pow(power)
        }

        pub fn active_nodes(&self) -> Vec<Ocnode> {
            let mut found: Vec<Ocnode> = vec![];

            if self.active {
                found.push(self.clone());
            }
            if self.has_children {
                let squirts = self.children.each_ref();

                for index in 0..8 {
                    match squirts[index] {
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

        pub fn clear(&mut self) {
            self.active = false;

            let squirts = self.children.each_mut();

            for index in 0..8 {
                match squirts[index] {
                    None => {}
                    Some(squirt) => {
                        squirt.clear();
                    }
                };
            }
        }

        pub fn apply(&mut self, node: &Ocnode) {
            if node.x_index == self.x_index
                && node.y_index == self.y_index
                && node.z_index == self.z_index
                && node.sub_division_level == self.sub_division_level
            {
                // We got a match. Apply it.
                self.active = node.active;
                self.color = node.color;
            }
            let squirts = self.children.each_mut();

            for index in 0..8 {
                match squirts[index] {
                    None => {}
                    Some(squirt) => {
                        squirt.apply(node);
                    }
                };
            }
        }

        /**
         * Set the active state to match the combined active state of all children.
         */
        pub fn optimise(&mut self) {
            if self.has_children {
                // Optimize leaf first then move up the tree.
                let squirts = self.children.each_mut();
                for child in squirts {
                    match child {
                        None => {}
                        Some(down) => {
                            down.optimise();
                        }
                    }
                }

                let squirts = self.children.each_mut();
                let has_hole = squirts
                    .into_iter()
                    .any(|child| !child.as_ref().expect("child").active);
                let squirts = self.children.each_mut();
                let mut colour = [0.0, 0.0, 0.0, 0.0];
                match squirts[0] {
                    None => {
                        log::debug!("Should not get here")
                    }
                    Some(node) => {
                        colour = node.color;
                    }
                };
                let squirts = self.children.each_mut();
                let not_uniform_color = squirts
                    .into_iter()
                    .any(|child| {
                        let compare = child.as_ref().expect("child").color;

                        compare[0] != colour[0] || compare[1] != colour[1] || compare[2] != colour[2] || compare[3] != colour[3]
                    });

                self.active = !has_hole && !not_uniform_color;
                let first = self.children.first();

                match first {
                    None => {}
                    Some(node) => {
                        self.color = node.as_ref().unwrap().color;
                    }
                }
            }
        }

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

            for index in 0..8 {
                match squirts[index] {
                    None => {}
                    Some(node) => {
                        if !node.all_voxels_active(positions) {
                            return false;
                        }
                    }
                };
            }

            return true;
        }

        pub fn toggle_voxel(&mut self, position: [i32; 3], value: bool, color: [f32; 4]) {
            if self.x_index == position[0]
                && self.y_index == position[1]
                && self.z_index == position[2]
                && self.sub_division_level == LEVELS
            {
                self.active = value;
                self.color = color;
            }
            let squirts = self.children.each_mut();

            for index in 0..8 {
                match squirts[index] {
                    None => {}
                    Some(node) => {
                        node.toggle_voxel(position, value, color);
                    }
                };
            }
        }

        pub fn drawables(&mut self) -> Vec<Cube> {
            if self.has_children {
                if self.active {
                    let scale = self.resolution(self.sub_division_level) as f32;
                    let mut cube = Cube::new();

                    cube.color = self.color;
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

                    for index in 0..8 {
                        match squirts[index] {
                            None => {}
                            Some(node) => {
                                let mut cube = node.drawables();

                                child_cubes.append(&mut cube);
                            }
                        };
                    }
                    child_cubes
                }
            } else {
                if self.active {
                    let scale = 1.0;
                    let mut cube = Cube::new();

                    cube.color = self.color;
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
        }

        pub fn decimate(&mut self, sub_division_level: u32) {
            if sub_division_level - 1 > 0 {
                self.subdivide();

                let squirts = self.children.each_mut();

                for index in 0..8 {
                    match squirts[index] {
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
            }));
        }
    }
}
