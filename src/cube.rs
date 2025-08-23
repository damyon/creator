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
}

use crate::drawable::Drawable;

impl Cube {
    /// Create a new default cube.
    pub const fn new() -> Cube {
        Cube {
            vertices_count: 108,
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
            occluded -= 18;
        }
        if self.left_occluded {
            occluded -= 18;
        }
        if self.right_occluded {
            occluded -= 18;
        }
        if self.front_occluded {
            occluded -= 18;
        }
        if self.back_occluded {
            occluded -= 18;
        }
        if self.top_occluded {
            occluded -= 18;
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
        let ldf = [self.floor, self.floor, self.floor];
        let luf = [self.floor, self.scale, self.floor];
        let ldb = [self.floor, self.floor, self.scale];
        let lub = [self.floor, self.scale, self.scale];
        let rdf = [self.scale, self.floor, self.floor];
        let ruf = [self.scale, self.scale, self.floor];
        let rdb = [self.scale, self.floor, self.scale];
        let rub = [self.scale, self.scale, self.scale];

        let mut index: usize = 0;
        let mut increment = || -> usize {
            let result = index;
            index += 1;
            result
        };

        let mut vertices = [0.0; 108];
        // Bottom
        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = rdf[0];
        vertices[increment()] = rdf[1];
        vertices[increment()] = rdf[2];
        vertices[increment()] = rdb[0];
        vertices[increment()] = rdb[1];
        vertices[increment()] = rdb[2];

        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = rdb[0];
        vertices[increment()] = rdb[1];
        vertices[increment()] = rdb[2];
        vertices[increment()] = ldb[0];
        vertices[increment()] = ldb[1];
        vertices[increment()] = ldb[2];

        // Left
        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = ldb[0];
        vertices[increment()] = ldb[1];
        vertices[increment()] = ldb[2];
        vertices[increment()] = lub[0];
        vertices[increment()] = lub[1];
        vertices[increment()] = lub[2];

        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = lub[0];
        vertices[increment()] = lub[1];
        vertices[increment()] = lub[2];
        vertices[increment()] = luf[0];
        vertices[increment()] = luf[1];
        vertices[increment()] = luf[2];
        // Right
        vertices[increment()] = rdf[0];
        vertices[increment()] = rdf[1];
        vertices[increment()] = rdf[2];
        vertices[increment()] = ruf[0];
        vertices[increment()] = ruf[1];
        vertices[increment()] = ruf[2];
        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];

        vertices[increment()] = rdf[0];
        vertices[increment()] = rdf[1];
        vertices[increment()] = rdf[2];
        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];
        vertices[increment()] = rdb[0];
        vertices[increment()] = rdb[1];
        vertices[increment()] = rdb[2];
        // Back
        vertices[increment()] = ldb[0];
        vertices[increment()] = ldb[1];
        vertices[increment()] = ldb[2];
        vertices[increment()] = rdb[0];
        vertices[increment()] = rdb[1];
        vertices[increment()] = rdb[2];
        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];

        vertices[increment()] = ldb[0];
        vertices[increment()] = ldb[1];
        vertices[increment()] = ldb[2];
        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];
        vertices[increment()] = lub[0];
        vertices[increment()] = lub[1];
        vertices[increment()] = lub[2];

        // Front
        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = luf[0];
        vertices[increment()] = luf[1];
        vertices[increment()] = luf[2];
        vertices[increment()] = ruf[0];
        vertices[increment()] = ruf[1];
        vertices[increment()] = ruf[2];

        vertices[increment()] = ldf[0];
        vertices[increment()] = ldf[1];
        vertices[increment()] = ldf[2];
        vertices[increment()] = ruf[0];
        vertices[increment()] = ruf[1];
        vertices[increment()] = ruf[2];
        vertices[increment()] = rdf[0];
        vertices[increment()] = rdf[1];
        vertices[increment()] = rdf[2];
        // Top
        vertices[increment()] = luf[0];
        vertices[increment()] = luf[1];
        vertices[increment()] = luf[2];
        vertices[increment()] = lub[0];
        vertices[increment()] = lub[1];
        vertices[increment()] = lub[2];
        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];

        vertices[increment()] = luf[0];
        vertices[increment()] = luf[1];
        vertices[increment()] = luf[2];
        vertices[increment()] = rub[0];
        vertices[increment()] = rub[1];
        vertices[increment()] = rub[2];
        vertices[increment()] = ruf[0];
        vertices[increment()] = ruf[1];
        vertices[increment()] = ruf[2];

        let bottom = &vertices[0..18];
        let left = &vertices[18..36];
        let right = &vertices[36..54];
        let back = &vertices[54..72];
        let front = &vertices[72..90];
        let top = &vertices[90..108];
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

        let left = [-1.0, 0.0, 0.0];
        let down = [0.0, -1.0, 0.0];
        let front = [0.0, 0.0, -1.0];
        let right = [1.0, 0.0, 0.0];
        let up = [0.0, 1.0, 0.0];
        let back = [0.0, 0.0, 1.0];

        let mut normals = [0.0; 108];

        // Bottom
        normals[normal_increment()] = down[0];
        normals[normal_increment()] = down[1];
        normals[normal_increment()] = down[2];
        normals[normal_increment()] = down[0];
        normals[normal_increment()] = down[1];
        normals[normal_increment()] = down[2];
        normals[normal_increment()] = down[0];
        normals[normal_increment()] = down[1];
        normals[normal_increment()] = down[2];

        normals[normal_increment()] = down[0];
        normals[normal_increment()] = down[1];
        normals[normal_increment()] = down[2];
        normals[normal_increment()] = down[0];
        normals[normal_increment()] = down[1];
        normals[normal_increment()] = down[2];
        normals[normal_increment()] = down[0];
        normals[normal_increment()] = down[1];
        normals[normal_increment()] = down[2];

        // Left
        normals[normal_increment()] = left[0];
        normals[normal_increment()] = left[1];
        normals[normal_increment()] = left[2];
        normals[normal_increment()] = left[0];
        normals[normal_increment()] = left[1];
        normals[normal_increment()] = left[2];
        normals[normal_increment()] = left[0];
        normals[normal_increment()] = left[1];
        normals[normal_increment()] = left[2];

        normals[normal_increment()] = left[0];
        normals[normal_increment()] = left[1];
        normals[normal_increment()] = left[2];
        normals[normal_increment()] = left[0];
        normals[normal_increment()] = left[1];
        normals[normal_increment()] = left[2];
        normals[normal_increment()] = left[0];
        normals[normal_increment()] = left[1];
        normals[normal_increment()] = left[2];

        // Right
        normals[normal_increment()] = right[0];
        normals[normal_increment()] = right[1];
        normals[normal_increment()] = right[2];
        normals[normal_increment()] = right[0];
        normals[normal_increment()] = right[1];
        normals[normal_increment()] = right[2];
        normals[normal_increment()] = right[0];
        normals[normal_increment()] = right[1];
        normals[normal_increment()] = right[2];

        normals[normal_increment()] = right[0];
        normals[normal_increment()] = right[1];
        normals[normal_increment()] = right[2];
        normals[normal_increment()] = right[0];
        normals[normal_increment()] = right[1];
        normals[normal_increment()] = right[2];
        normals[normal_increment()] = right[0];
        normals[normal_increment()] = right[1];
        normals[normal_increment()] = right[2];

        // Back
        normals[normal_increment()] = back[0];
        normals[normal_increment()] = back[1];
        normals[normal_increment()] = back[2];
        normals[normal_increment()] = back[0];
        normals[normal_increment()] = back[1];
        normals[normal_increment()] = back[2];
        normals[normal_increment()] = back[0];
        normals[normal_increment()] = back[1];
        normals[normal_increment()] = back[2];

        normals[normal_increment()] = back[0];
        normals[normal_increment()] = back[1];
        normals[normal_increment()] = back[2];
        normals[normal_increment()] = back[0];
        normals[normal_increment()] = back[1];
        normals[normal_increment()] = back[2];
        normals[normal_increment()] = back[0];
        normals[normal_increment()] = back[1];
        normals[normal_increment()] = back[2];
        // Front
        normals[normal_increment()] = front[0];
        normals[normal_increment()] = front[1];
        normals[normal_increment()] = front[2];
        normals[normal_increment()] = front[0];
        normals[normal_increment()] = front[1];
        normals[normal_increment()] = front[2];
        normals[normal_increment()] = front[0];
        normals[normal_increment()] = front[1];
        normals[normal_increment()] = front[2];

        normals[normal_increment()] = front[0];
        normals[normal_increment()] = front[1];
        normals[normal_increment()] = front[2];
        normals[normal_increment()] = front[0];
        normals[normal_increment()] = front[1];
        normals[normal_increment()] = front[2];
        normals[normal_increment()] = front[0];
        normals[normal_increment()] = front[1];
        normals[normal_increment()] = front[2];
        // Top
        normals[normal_increment()] = up[0];
        normals[normal_increment()] = up[1];
        normals[normal_increment()] = up[2];
        normals[normal_increment()] = up[0];
        normals[normal_increment()] = up[1];
        normals[normal_increment()] = up[2];
        normals[normal_increment()] = up[0];
        normals[normal_increment()] = up[1];
        normals[normal_increment()] = up[2];

        normals[normal_increment()] = up[0];
        normals[normal_increment()] = up[1];
        normals[normal_increment()] = up[2];
        normals[normal_increment()] = up[0];
        normals[normal_increment()] = up[1];
        normals[normal_increment()] = up[2];
        normals[normal_increment()] = up[0];
        normals[normal_increment()] = up[1];
        normals[normal_increment()] = up[2];

        let bottom = &normals[0..18];
        let left = &normals[18..36];
        let right = &normals[36..54];
        let front = &normals[54..72];
        let back = &normals[72..90];
        let top = &normals[90..108];
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
