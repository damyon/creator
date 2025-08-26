/// A cube is a drawable item that can be positioned, rotated and scaled.
#[derive(Copy, Clone)]
pub struct Cube {
    pub vertices_count: u16,
    pub translation: [f32; 3],
    pub rotation: [f32; 3],
    pub color: [f32; 4],
    pub scale: f32,
    pub center: f32,
    pub floor: f32,
    pub fluid: i32,
    pub noise: i32,
    pub bottom_occluded: bool,
    pub left_occluded: bool,
    pub right_occluded: bool,
    pub front_occluded: bool,
    pub back_occluded: bool,
    pub top_occluded: bool,
    pub smooth: bool,
}

use nalgebra_glm::Vec3;

use crate::drawable::Drawable;

impl Cube {
    /// Create a new default cube.
    pub const fn new() -> Cube {
        Cube {
            vertices_count: 216,
            translation: [0.0; 3],
            rotation: [0.0; 3],
            color: [0.3, 0.3, 0.1, 1.0],
            scale: 0.9999, // The scale is slightly smaller than 1 to prevent z-fighting
            center: 0.5,
            floor: 0.0001,
            fluid: 0,
            noise: 0,
            bottom_occluded: false,
            left_occluded: false,
            right_occluded: false,
            front_occluded: false,
            back_occluded: false,
            top_occluded: false,
            smooth: false,
        }
    }
}

impl Drawable for Cube {
    /// Init a new cube so it's ready to draw.
    fn init(&mut self) {}

    /// A cube always has the same number of vertices minus oclusion
    fn count_vertices(&self) -> u16 {
        let mut occluded = self.vertices_count;
        if self.bottom_occluded {
            occluded -= 36;
        }
        if self.left_occluded {
            occluded -= 36;
        }
        if self.right_occluded {
            occluded -= 36;
        }
        if self.front_occluded {
            occluded -= 36;
        }
        if self.back_occluded {
            occluded -= 36;
        }
        if self.top_occluded {
            occluded -= 36;
        }
        occluded
    }

    /// We can move a cube
    fn translation(&self) -> &[f32; 3] {
        &self.translation
    }

    /// Cubes have a colour - including alphas.
    fn color(&self) -> &[f32; 4] {
        &self.color
    }

    fn fluid(&self) -> i32 {
        self.fluid
    }

    fn noise(&self) -> i32 {
        self.noise
    }

    /// Move a cube.
    fn translate(&mut self, amount: [f32; 3]) {
        self.translation[0] += amount[0];
        self.translation[1] += amount[1];
        self.translation[2] += amount[2];
    }

    /// Rotate a cube.
    fn rotate(&mut self, amount: [f32; 3]) {
        self.rotation[0] += amount[0];
        self.rotation[1] += amount[1];
        self.rotation[2] += amount[2];
    }

    /// Get the current rotation.
    fn rotation(&self) -> &[f32; 3] {
        &self.rotation
    }

    /// Get an array of vertices.
    fn vertices(&self) -> Vec<f32> {
        // We really have 8 points.
        // Start by calcuting the points.
        // naming is l/r u/d f/b
        // which is -x/+x -y/+y / -z/+z
        let buldge = 0.6;
        let lc = [
            if self.smooth
                && !self.front_occluded
                && !self.bottom_occluded
                && !self.left_occluded
                && !self.back_occluded
                && !self.top_occluded
            {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            self.center,
            self.center,
        ];
        let rc = [
            if self.smooth
                && !self.front_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.back_occluded
                && !self.top_occluded
            {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            self.center,
            self.center,
        ];
        let fc = [
            self.center,
            self.center,
            if self.smooth
                && !self.front_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.top_occluded
            {
                self.center - self.center * buldge
            } else {
                self.floor
            },
        ];
        let bc = [
            self.center,
            self.center,
            if self.smooth
                && !self.back_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.top_occluded
            {
                self.center + self.center * buldge
            } else {
                self.scale
            },
        ];
        let dc = [
            self.center,
            if self.smooth
                && !self.back_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.front_occluded
            {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            self.center,
        ];

        let uc = [
            self.center,
            if self.smooth
                && !self.back_occluded
                && !self.top_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.front_occluded
            {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            self.center,
        ];

        let ldf = [
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
        ];
        let luf = [
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.left_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
        ];
        let ldb = [
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
        ];
        let lub = [
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.left_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.left_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
        ];
        let rdf = [
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.right_occluded
            {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.right_occluded
            {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.right_occluded
            {
                self.center - self.center * buldge
            } else {
                self.floor
            },
        ];
        let ruf = [
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.right_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
        ];
        let rdb = [
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.right_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
        ];
        let rub = [
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
        ];

        let mut index: usize = 0;
        let mut increment = || -> usize {
            let result = index;
            index += 1;
            result
        };

        let mut vertices = [0.0; 216];
        // Bottom
        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = rdf[0];
        vertices[increment()] = rdf[1];
        vertices[increment()] = rdf[2];
        vertices[increment()] = dc[0];
        vertices[increment()] = dc[1];
        vertices[increment()] = dc[2];

        vertices[increment()] = rdf[0];
        vertices[increment()] = rdf[1];
        vertices[increment()] = rdf[2];
        vertices[increment()] = rdb[0];
        vertices[increment()] = rdb[1];
        vertices[increment()] = rdb[2];
        vertices[increment()] = dc[0];
        vertices[increment()] = dc[1];
        vertices[increment()] = dc[2];

        vertices[increment()] = rdb[0];
        vertices[increment()] = rdb[1];
        vertices[increment()] = rdb[2];
        vertices[increment()] = ldb[0];
        vertices[increment()] = ldb[1];
        vertices[increment()] = ldb[2];
        vertices[increment()] = dc[0];
        vertices[increment()] = dc[1];
        vertices[increment()] = dc[2];

        vertices[increment()] = ldb[0];
        vertices[increment()] = ldb[1];
        vertices[increment()] = ldb[2];
        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = dc[0];
        vertices[increment()] = dc[1];
        vertices[increment()] = dc[2];

        // Left
        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = ldb[0];
        vertices[increment()] = ldb[1];
        vertices[increment()] = ldb[2];
        vertices[increment()] = lc[0];
        vertices[increment()] = lc[1];
        vertices[increment()] = lc[2];

        vertices[increment()] = luf[0];
        vertices[increment()] = luf[1];
        vertices[increment()] = luf[2];
        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = lc[0];
        vertices[increment()] = lc[1];
        vertices[increment()] = lc[2];

        vertices[increment()] = lub[0];
        vertices[increment()] = lub[1];
        vertices[increment()] = lub[2];
        vertices[increment()] = luf[0];
        vertices[increment()] = luf[1];
        vertices[increment()] = luf[2];
        vertices[increment()] = lc[0];
        vertices[increment()] = lc[1];
        vertices[increment()] = lc[2];

        vertices[increment()] = ldb[0];
        vertices[increment()] = ldb[1];
        vertices[increment()] = ldb[2];
        vertices[increment()] = lub[0];
        vertices[increment()] = lub[1];
        vertices[increment()] = lub[2];
        vertices[increment()] = lc[0];
        vertices[increment()] = lc[1];
        vertices[increment()] = lc[2];
        // Right
        vertices[increment()] = rdf[0];
        vertices[increment()] = rdf[1];
        vertices[increment()] = rdf[2];
        vertices[increment()] = ruf[0];
        vertices[increment()] = ruf[1];
        vertices[increment()] = ruf[2];
        vertices[increment()] = rc[0];
        vertices[increment()] = rc[1];
        vertices[increment()] = rc[2];

        vertices[increment()] = ruf[0];
        vertices[increment()] = ruf[1];
        vertices[increment()] = ruf[2];
        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];
        vertices[increment()] = rc[0];
        vertices[increment()] = rc[1];
        vertices[increment()] = rc[2];

        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];
        vertices[increment()] = rdb[0];
        vertices[increment()] = rdb[1];
        vertices[increment()] = rdb[2];
        vertices[increment()] = rc[0];
        vertices[increment()] = rc[1];
        vertices[increment()] = rc[2];

        vertices[increment()] = rdb[0];
        vertices[increment()] = rdb[1];
        vertices[increment()] = rdb[2];
        vertices[increment()] = rdf[0];
        vertices[increment()] = rdf[1];
        vertices[increment()] = rdf[2];
        vertices[increment()] = rc[0];
        vertices[increment()] = rc[1];
        vertices[increment()] = rc[2];
        // Back
        vertices[increment()] = ldb[0];
        vertices[increment()] = ldb[1];
        vertices[increment()] = ldb[2];
        vertices[increment()] = rdb[0];
        vertices[increment()] = rdb[1];
        vertices[increment()] = rdb[2];
        vertices[increment()] = bc[0];
        vertices[increment()] = bc[1];
        vertices[increment()] = bc[2];

        vertices[increment()] = rdb[0];
        vertices[increment()] = rdb[1];
        vertices[increment()] = rdb[2];
        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];
        vertices[increment()] = bc[0];
        vertices[increment()] = bc[1];
        vertices[increment()] = bc[2];

        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];
        vertices[increment()] = lub[0];
        vertices[increment()] = lub[1];
        vertices[increment()] = lub[2];
        vertices[increment()] = bc[0];
        vertices[increment()] = bc[1];
        vertices[increment()] = bc[2];

        vertices[increment()] = lub[0];
        vertices[increment()] = lub[1];
        vertices[increment()] = lub[2];
        vertices[increment()] = ldb[0];
        vertices[increment()] = ldb[1];
        vertices[increment()] = ldb[2];
        vertices[increment()] = bc[0];
        vertices[increment()] = bc[1];
        vertices[increment()] = bc[2];

        // Front
        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = luf[0];
        vertices[increment()] = luf[1];
        vertices[increment()] = luf[2];
        vertices[increment()] = fc[0];
        vertices[increment()] = fc[1];
        vertices[increment()] = fc[2];

        vertices[increment()] = luf[0];
        vertices[increment()] = luf[1];
        vertices[increment()] = luf[2];
        vertices[increment()] = ruf[0];
        vertices[increment()] = ruf[1];
        vertices[increment()] = ruf[2];
        vertices[increment()] = fc[0];
        vertices[increment()] = fc[1];
        vertices[increment()] = fc[2];

        vertices[increment()] = ruf[0];
        vertices[increment()] = ruf[1];
        vertices[increment()] = ruf[2];
        vertices[increment()] = rdf[0];
        vertices[increment()] = rdf[1];
        vertices[increment()] = rdf[2];
        vertices[increment()] = fc[0];
        vertices[increment()] = fc[1];
        vertices[increment()] = fc[2];

        vertices[increment()] = rdf[0];
        vertices[increment()] = rdf[1];
        vertices[increment()] = rdf[2];
        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = fc[0];
        vertices[increment()] = fc[1];
        vertices[increment()] = fc[2];

        // Top
        vertices[increment()] = luf[0];
        vertices[increment()] = luf[1];
        vertices[increment()] = luf[2];
        vertices[increment()] = lub[0];
        vertices[increment()] = lub[1];
        vertices[increment()] = lub[2];
        vertices[increment()] = uc[0];
        vertices[increment()] = uc[1];
        vertices[increment()] = uc[2];

        vertices[increment()] = lub[0];
        vertices[increment()] = lub[1];
        vertices[increment()] = lub[2];
        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];
        vertices[increment()] = uc[0];
        vertices[increment()] = uc[1];
        vertices[increment()] = uc[2];

        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];
        vertices[increment()] = ruf[0];
        vertices[increment()] = ruf[1];
        vertices[increment()] = ruf[2];
        vertices[increment()] = uc[0];
        vertices[increment()] = uc[1];
        vertices[increment()] = uc[2];

        vertices[increment()] = ruf[0];
        vertices[increment()] = ruf[1];
        vertices[increment()] = ruf[2];
        vertices[increment()] = luf[0];
        vertices[increment()] = luf[1];
        vertices[increment()] = luf[2];
        vertices[increment()] = uc[0];
        vertices[increment()] = uc[1];
        vertices[increment()] = uc[2];

        let bottom = &vertices[0..36];
        let left = &vertices[36..72];
        let right = &vertices[72..108];
        let back = &vertices[108..144];
        let front = &vertices[144..180];
        let top = &vertices[180..216];
        let mut valid = vec![];

        if !self.bottom_occluded {
            valid.extend_from_slice(bottom);
        }
        if !self.left_occluded {
            valid.extend_from_slice(left);
        }
        if !self.right_occluded {
            valid.extend_from_slice(right);
        }
        if !self.front_occluded {
            valid.extend_from_slice(front);
        }
        if !self.back_occluded {
            valid.extend_from_slice(back);
        }
        if !self.top_occluded {
            valid.extend_from_slice(top);
        }

        valid
    }

    /// Get an array of normals.
    fn normals(&self) -> Vec<f32> {
        let mut normal_index: usize = 0;
        let mut normal_increment = || -> usize {
            let normal_result = normal_index;
            normal_index += 1;
            normal_result
        };

        let buldge = 0.6;
        let lc = [
            if self.smooth
                && !self.front_occluded
                && !self.bottom_occluded
                && !self.left_occluded
                && !self.back_occluded
                && !self.top_occluded
            {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            self.center,
            self.center,
        ];
        let rc = [
            if self.smooth
                && !self.front_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.back_occluded
                && !self.top_occluded
            {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            self.center,
            self.center,
        ];
        let fc = [
            self.center,
            self.center,
            if self.smooth
                && !self.front_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.top_occluded
            {
                self.center - self.center * buldge
            } else {
                self.floor
            },
        ];
        let bc = [
            self.center,
            self.center,
            if self.smooth
                && !self.back_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.top_occluded
            {
                self.center + self.center * buldge
            } else {
                self.scale
            },
        ];
        let dc = [
            self.center,
            if self.smooth
                && !self.back_occluded
                && !self.bottom_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.front_occluded
            {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            self.center,
        ];

        let uc = [
            self.center,
            if self.smooth
                && !self.back_occluded
                && !self.top_occluded
                && !self.right_occluded
                && !self.left_occluded
                && !self.front_occluded
            {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            self.center,
        ];

        let ldf = [
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
        ];
        let luf = [
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.left_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
        ];
        let ldb = [
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.left_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
        ];
        let lub = [
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.left_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.left_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.left_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
        ];
        let rdf = [
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.right_occluded
            {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.right_occluded
            {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.front_occluded && !self.bottom_occluded && !self.right_occluded
            {
                self.center - self.center * buldge
            } else {
                self.floor
            },
        ];
        let ruf = [
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.front_occluded && !self.top_occluded && !self.right_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
        ];
        let rdb = [
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.right_occluded {
                self.center - self.center * buldge
            } else {
                self.floor
            },
            if self.smooth && !self.back_occluded && !self.bottom_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
        ];
        let rub = [
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
            if self.smooth && !self.back_occluded && !self.top_occluded && !self.right_occluded {
                self.center + self.center * buldge
            } else {
                self.scale
            },
        ];

        let mut normals = [0.0; 216];

        // Bottom
        // ldf, rdf, dc
        let b11 = Vec3::new(ldf[0] - dc[0], ldf[1] - dc[1], ldf[2] - dc[2]);
        let b12 = Vec3::new(rdf[0] - dc[0], rdf[1] - dc[1], rdf[2] - dc[2]);
        let bc1 = b11.cross(&b12);
        normals[normal_increment()] = bc1[0];
        normals[normal_increment()] = bc1[1];
        normals[normal_increment()] = bc1[2];
        normals[normal_increment()] = bc1[0];
        normals[normal_increment()] = bc1[1];
        normals[normal_increment()] = bc1[2];
        normals[normal_increment()] = bc1[0];
        normals[normal_increment()] = bc1[1];
        normals[normal_increment()] = bc1[2];

        // rdf, rdb, dc
        let b21 = Vec3::new(rdf[0] - dc[0], rdf[1] - dc[1], rdf[2] - dc[2]);
        let b22 = Vec3::new(rdb[0] - dc[0], rdb[1] - dc[1], rdb[2] - dc[2]);
        let bc2 = b21.cross(&b22);

        normals[normal_increment()] = bc2[0];
        normals[normal_increment()] = bc2[1];
        normals[normal_increment()] = bc2[2];
        normals[normal_increment()] = bc2[0];
        normals[normal_increment()] = bc2[1];
        normals[normal_increment()] = bc2[2];
        normals[normal_increment()] = bc2[0];
        normals[normal_increment()] = bc2[1];
        normals[normal_increment()] = bc2[2];

        // rdb, ldb, dc
        let b31 = Vec3::new(rdb[0] - dc[0], rdb[1] - dc[1], rdb[2] - dc[2]);
        let b32 = Vec3::new(ldb[0] - dc[0], ldb[1] - dc[1], ldb[2] - dc[2]);
        let bc3 = b31.cross(&b32);

        normals[normal_increment()] = bc3[0];
        normals[normal_increment()] = bc3[1];
        normals[normal_increment()] = bc3[2];
        normals[normal_increment()] = bc3[0];
        normals[normal_increment()] = bc3[1];
        normals[normal_increment()] = bc3[2];
        normals[normal_increment()] = bc3[0];
        normals[normal_increment()] = bc3[1];
        normals[normal_increment()] = bc3[2];

        // ldb, ldf, dc
        let b41 = Vec3::new(ldb[0] - dc[0], ldb[1] - dc[1], ldb[2] - dc[2]);
        let b42 = Vec3::new(ldf[0] - dc[0], ldf[1] - dc[1], ldf[2] - dc[2]);
        let bc4 = b41.cross(&b42);

        normals[normal_increment()] = bc4[0];
        normals[normal_increment()] = bc4[1];
        normals[normal_increment()] = bc4[2];
        normals[normal_increment()] = bc4[0];
        normals[normal_increment()] = bc4[1];
        normals[normal_increment()] = bc4[2];
        normals[normal_increment()] = bc4[0];
        normals[normal_increment()] = bc4[1];
        normals[normal_increment()] = bc4[2];

        // Left
        // ldf, ldb, lc
        let l11 = Vec3::new(ldf[0] - lc[0], ldf[1] - lc[1], ldf[2] - lc[2]);
        let l12 = Vec3::new(ldb[0] - lc[0], ldb[1] - lc[1], ldb[2] - lc[2]);
        let lc1 = l11.cross(&l12);

        normals[normal_increment()] = lc1[0];
        normals[normal_increment()] = lc1[1];
        normals[normal_increment()] = lc1[2];
        normals[normal_increment()] = lc1[0];
        normals[normal_increment()] = lc1[1];
        normals[normal_increment()] = lc1[2];
        normals[normal_increment()] = lc1[0];
        normals[normal_increment()] = lc1[1];
        normals[normal_increment()] = lc1[2];

        // luf, ldf, lc
        let l21 = Vec3::new(luf[0] - lc[0], luf[1] - lc[1], luf[2] - lc[2]);
        let l22 = Vec3::new(ldf[0] - lc[0], ldf[1] - lc[1], ldf[2] - lc[2]);
        let lc2 = l21.cross(&l22);

        normals[normal_increment()] = lc2[0];
        normals[normal_increment()] = lc2[1];
        normals[normal_increment()] = lc2[2];
        normals[normal_increment()] = lc2[0];
        normals[normal_increment()] = lc2[1];
        normals[normal_increment()] = lc2[2];
        normals[normal_increment()] = lc2[0];
        normals[normal_increment()] = lc2[1];
        normals[normal_increment()] = lc2[2];

        // lub, luf, lc
        let l31 = Vec3::new(lub[0] - lc[0], lub[1] - lc[1], lub[2] - lc[2]);
        let l32 = Vec3::new(luf[0] - lc[0], luf[1] - lc[1], luf[2] - lc[2]);
        let lc3 = l31.cross(&l32);

        normals[normal_increment()] = lc3[0];
        normals[normal_increment()] = lc3[1];
        normals[normal_increment()] = lc3[2];
        normals[normal_increment()] = lc3[0];
        normals[normal_increment()] = lc3[1];
        normals[normal_increment()] = lc3[2];
        normals[normal_increment()] = lc3[0];
        normals[normal_increment()] = lc3[1];
        normals[normal_increment()] = lc3[2];

        // ldb, lub, lc
        let l41 = Vec3::new(ldb[0] - lc[0], ldb[1] - lc[1], ldb[2] - lc[2]);
        let l42 = Vec3::new(lub[0] - lc[0], lub[1] - lc[1], lub[2] - lc[2]);
        let lc4 = l41.cross(&l42);

        normals[normal_increment()] = lc4[0];
        normals[normal_increment()] = lc4[1];
        normals[normal_increment()] = lc4[2];
        normals[normal_increment()] = lc4[0];
        normals[normal_increment()] = lc4[1];
        normals[normal_increment()] = lc4[2];
        normals[normal_increment()] = lc4[0];
        normals[normal_increment()] = lc4[1];
        normals[normal_increment()] = lc4[2];

        // Right
        // rdf, ruf, rc
        let r11 = Vec3::new(rdf[0] - rc[0], rdf[1] - rc[1], rdf[2] - rc[2]);
        let r12 = Vec3::new(ruf[0] - rc[0], ruf[1] - rc[1], ruf[2] - rc[2]);
        let rc1 = r11.cross(&r12);
        normals[normal_increment()] = rc1[0];
        normals[normal_increment()] = rc1[1];
        normals[normal_increment()] = rc1[2];
        normals[normal_increment()] = rc1[0];
        normals[normal_increment()] = rc1[1];
        normals[normal_increment()] = rc1[2];
        normals[normal_increment()] = rc1[0];
        normals[normal_increment()] = rc1[1];
        normals[normal_increment()] = rc1[2];

        // ruf, rub, rc
        let r21 = Vec3::new(ruf[0] - rc[0], ruf[1] - rc[1], ruf[2] - rc[2]);
        let r22 = Vec3::new(rub[0] - rc[0], rub[1] - rc[1], rub[2] - rc[2]);
        let rc2 = r21.cross(&r22);
        normals[normal_increment()] = rc2[0];
        normals[normal_increment()] = rc2[1];
        normals[normal_increment()] = rc2[2];
        normals[normal_increment()] = rc2[0];
        normals[normal_increment()] = rc2[1];
        normals[normal_increment()] = rc2[2];
        normals[normal_increment()] = rc2[0];
        normals[normal_increment()] = rc2[1];
        normals[normal_increment()] = rc2[2];

        // rub, rdb, rc
        let r31 = Vec3::new(rub[0] - rc[0], rub[1] - rc[1], rub[2] - rc[2]);
        let r32 = Vec3::new(rdb[0] - rc[0], rdb[1] - rc[1], rdb[2] - rc[2]);
        let rc3 = r31.cross(&r32);
        normals[normal_increment()] = rc3[0];
        normals[normal_increment()] = rc3[1];
        normals[normal_increment()] = rc3[2];
        normals[normal_increment()] = rc3[0];
        normals[normal_increment()] = rc3[1];
        normals[normal_increment()] = rc3[2];
        normals[normal_increment()] = rc3[0];
        normals[normal_increment()] = rc3[1];
        normals[normal_increment()] = rc3[2];

        // rdb, rdf, rc
        let r41 = Vec3::new(rdb[0] - rc[0], rdb[1] - rc[1], rdb[2] - rc[2]);
        let r42 = Vec3::new(rdf[0] - rc[0], rdf[1] - rc[1], rdf[2] - rc[2]);
        let rc4 = r41.cross(&r42);
        normals[normal_increment()] = rc4[0];
        normals[normal_increment()] = rc4[1];
        normals[normal_increment()] = rc4[2];
        normals[normal_increment()] = rc4[0];
        normals[normal_increment()] = rc4[1];
        normals[normal_increment()] = rc4[2];
        normals[normal_increment()] = rc4[0];
        normals[normal_increment()] = rc4[1];
        normals[normal_increment()] = rc4[2];

        // Back
        // ldb, rdb, bc
        let b11 = Vec3::new(rdb[0] - bc[0], rdb[1] - bc[1], rdb[2] - bc[2]);
        let b12 = Vec3::new(rub[0] - bc[0], rub[1] - bc[1], rub[2] - bc[2]);
        let bc1 = b11.cross(&b12);

        normals[normal_increment()] = bc1[0];
        normals[normal_increment()] = bc1[1];
        normals[normal_increment()] = bc1[2];
        normals[normal_increment()] = bc1[0];
        normals[normal_increment()] = bc1[1];
        normals[normal_increment()] = bc1[2];
        normals[normal_increment()] = bc1[0];
        normals[normal_increment()] = bc1[1];
        normals[normal_increment()] = bc1[2];

        // rdb, rub, bc
        let b21 = Vec3::new(rdb[0] - bc[0], rdb[1] - bc[1], rdb[2] - bc[2]);
        let b22 = Vec3::new(rub[0] - bc[0], rub[1] - bc[1], rub[2] - bc[2]);
        let bc2 = b21.cross(&b22);

        normals[normal_increment()] = bc2[0];
        normals[normal_increment()] = bc2[1];
        normals[normal_increment()] = bc2[2];
        normals[normal_increment()] = bc2[0];
        normals[normal_increment()] = bc2[1];
        normals[normal_increment()] = bc2[2];
        normals[normal_increment()] = bc2[0];
        normals[normal_increment()] = bc2[1];
        normals[normal_increment()] = bc2[2];

        // rub, lub, bc
        let b31 = Vec3::new(rub[0] - bc[0], rub[1] - bc[1], rub[2] - bc[2]);
        let b32 = Vec3::new(lub[0] - bc[0], lub[1] - bc[1], lub[2] - bc[2]);
        let bc3 = b31.cross(&b32);

        normals[normal_increment()] = bc3[0];
        normals[normal_increment()] = bc3[1];
        normals[normal_increment()] = bc3[2];
        normals[normal_increment()] = bc3[0];
        normals[normal_increment()] = bc3[1];
        normals[normal_increment()] = bc3[2];
        normals[normal_increment()] = bc3[0];
        normals[normal_increment()] = bc3[1];
        normals[normal_increment()] = bc3[2];

        // lub, ldb, bc
        let b41 = Vec3::new(lub[0] - bc[0], lub[1] - bc[1], lub[2] - bc[2]);
        let b42 = Vec3::new(ldb[0] - bc[0], ldb[1] - bc[1], ldb[2] - bc[2]);
        let bc4 = b41.cross(&b42);

        normals[normal_increment()] = bc4[0];
        normals[normal_increment()] = bc4[1];
        normals[normal_increment()] = bc4[2];
        normals[normal_increment()] = bc4[0];
        normals[normal_increment()] = bc4[1];
        normals[normal_increment()] = bc4[2];
        normals[normal_increment()] = bc4[0];
        normals[normal_increment()] = bc4[1];
        normals[normal_increment()] = bc4[2];

        // Front
        // ldf, luf, fc
        let f11 = Vec3::new(ldf[0] - fc[0], ldf[1] - fc[1], ldf[2] - fc[2]);
        let f12 = Vec3::new(luf[0] - fc[0], luf[1] - fc[1], luf[2] - fc[2]);
        let fc1 = f11.cross(&f12);

        normals[normal_increment()] = fc1[0];
        normals[normal_increment()] = fc1[1];
        normals[normal_increment()] = fc1[2];
        normals[normal_increment()] = fc1[0];
        normals[normal_increment()] = fc1[1];
        normals[normal_increment()] = fc1[2];
        normals[normal_increment()] = fc1[0];
        normals[normal_increment()] = fc1[1];
        normals[normal_increment()] = fc1[2];

        // luf, ruf, fc
        let f21 = Vec3::new(luf[0] - fc[0], luf[1] - fc[1], luf[2] - fc[2]);
        let f22 = Vec3::new(ruf[0] - fc[0], ruf[1] - fc[1], ruf[2] - fc[2]);
        let fc2 = f21.cross(&f22);

        normals[normal_increment()] = fc2[0];
        normals[normal_increment()] = fc2[1];
        normals[normal_increment()] = fc2[2];
        normals[normal_increment()] = fc2[0];
        normals[normal_increment()] = fc2[1];
        normals[normal_increment()] = fc2[2];
        normals[normal_increment()] = fc2[0];
        normals[normal_increment()] = fc2[1];
        normals[normal_increment()] = fc2[2];

        // ruf, rdf, fc
        let f31 = Vec3::new(ruf[0] - fc[0], ruf[1] - fc[1], ruf[2] - fc[2]);
        let f32 = Vec3::new(rdf[0] - fc[0], rdf[1] - fc[1], rdf[2] - fc[2]);
        let fc3 = f31.cross(&f32);
        normals[normal_increment()] = fc3[0];
        normals[normal_increment()] = fc3[1];
        normals[normal_increment()] = fc3[2];
        normals[normal_increment()] = fc3[0];
        normals[normal_increment()] = fc3[1];
        normals[normal_increment()] = fc3[2];
        normals[normal_increment()] = fc3[0];
        normals[normal_increment()] = fc3[1];
        normals[normal_increment()] = fc3[2];

        // rdf, ldf, fc
        let f41 = Vec3::new(rdf[0] - fc[0], rdf[1] - fc[1], rdf[2] - fc[2]);
        let f42 = Vec3::new(ldf[0] - fc[0], ldf[1] - fc[1], ldf[2] - fc[2]);
        let fc4 = f41.cross(&f42);
        normals[normal_increment()] = fc4[0];
        normals[normal_increment()] = fc4[1];
        normals[normal_increment()] = fc4[2];
        normals[normal_increment()] = fc4[0];
        normals[normal_increment()] = fc4[1];
        normals[normal_increment()] = fc4[2];
        normals[normal_increment()] = fc4[0];
        normals[normal_increment()] = fc4[1];
        normals[normal_increment()] = fc4[2];

        // Top
        // luf, lub, uc
        let t11 = Vec3::new(luf[0] - uc[0], luf[1] - uc[1], luf[2] - uc[2]);
        let t12 = Vec3::new(lub[0] - uc[0], lub[1] - uc[1], lub[2] - uc[2]);
        let tc1 = t11.cross(&t12);
        normals[normal_increment()] = tc1[0];
        normals[normal_increment()] = tc1[1];
        normals[normal_increment()] = tc1[2];
        normals[normal_increment()] = tc1[0];
        normals[normal_increment()] = tc1[1];
        normals[normal_increment()] = tc1[2];
        normals[normal_increment()] = tc1[0];
        normals[normal_increment()] = tc1[1];
        normals[normal_increment()] = tc1[2];

        // lub, rub, uc
        let t21 = Vec3::new(lub[0] - uc[0], lub[1] - uc[1], lub[2] - uc[2]);
        let t22 = Vec3::new(rub[0] - uc[0], rub[1] - uc[1], rub[2] - uc[2]);
        let tc2 = t21.cross(&t22);
        normals[normal_increment()] = tc2[0];
        normals[normal_increment()] = tc2[1];
        normals[normal_increment()] = tc2[2];
        normals[normal_increment()] = tc2[0];
        normals[normal_increment()] = tc2[1];
        normals[normal_increment()] = tc2[2];
        normals[normal_increment()] = tc2[0];
        normals[normal_increment()] = tc2[1];
        normals[normal_increment()] = tc2[2];

        // rub, ruf, uc
        let t31 = Vec3::new(rub[0] - uc[0], rub[1] - uc[1], rub[2] - uc[2]);
        let t32 = Vec3::new(ruf[0] - uc[0], ruf[1] - uc[1], ruf[2] - uc[2]);
        let tc3 = t31.cross(&t32);
        normals[normal_increment()] = tc3[0];
        normals[normal_increment()] = tc3[1];
        normals[normal_increment()] = tc3[2];
        normals[normal_increment()] = tc3[0];
        normals[normal_increment()] = tc3[1];
        normals[normal_increment()] = tc3[2];
        normals[normal_increment()] = tc3[0];
        normals[normal_increment()] = tc3[1];
        normals[normal_increment()] = tc3[2];

        // ruf, luf, uc
        let t41 = Vec3::new(ruf[0] - uc[0], ruf[1] - uc[1], ruf[2] - uc[2]);
        let t42 = Vec3::new(luf[0] - uc[0], luf[1] - uc[1], luf[2] - uc[2]);
        let tc4 = t41.cross(&t42);
        normals[normal_increment()] = tc4[0];
        normals[normal_increment()] = tc4[1];
        normals[normal_increment()] = tc4[2];
        normals[normal_increment()] = tc4[0];
        normals[normal_increment()] = tc4[1];
        normals[normal_increment()] = tc4[2];
        normals[normal_increment()] = tc4[0];
        normals[normal_increment()] = tc4[1];
        normals[normal_increment()] = tc4[2];

        let bottom = &normals[0..36];
        let left = &normals[36..72];
        let right = &normals[72..108];
        let front = &normals[108..144];
        let back = &normals[144..180];
        let top = &normals[180..216];
        let mut valid = vec![];

        if !self.bottom_occluded {
            valid.extend_from_slice(bottom);
        }
        if !self.left_occluded {
            valid.extend_from_slice(left);
        }
        if !self.right_occluded {
            valid.extend_from_slice(right);
        }
        if !self.front_occluded {
            valid.extend_from_slice(front);
        }
        if !self.back_occluded {
            valid.extend_from_slice(back);
        }
        if !self.top_occluded {
            valid.extend_from_slice(top);
        }
        valid
    }

    /// Calculate the distance between the cube and the camera.
    fn depth(&self, camera: [f32; 3]) -> f32 {
        ((self.translation[0] - camera[0]).powi(2)
            + (self.translation[1] - camera[1]).powi(2)
            + (self.translation[2] - camera[2]).powi(2))
        .sqrt()
    }
}
