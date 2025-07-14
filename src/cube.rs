/// A cube is a drawable item that can be positioned, rotated and scaled.
#[derive(Copy, Clone)]
pub struct Cube {
    pub vertices_count: u16,
    pub vertices: [f32; 108],
    pub normals: [f32; 108],
    pub translation: [f32; 3],
    pub rotation: [f32; 3],
    pub color: [f32; 4],
    pub scale: f32,
    pub floor: f32,
    pub fluid: i32,
    pub noise: i32,
}

use crate::drawable::Drawable;

impl Cube {
    /// Create a new default cube.
    pub const fn new() -> Cube {
        Cube {
            vertices_count: 108,
            vertices: [0.0; 108],
            normals: [0.0; 108],
            translation: [0.0; 3],
            rotation: [0.0; 3],
            color: [0.3, 0.3, 0.1, 1.0],
            scale: 0.9999, // The scale is slightly smaller than 1 to prevent z-fighting
            floor: 0.0001,
            fluid: 0,
            noise: 0,
        }
    }
}

impl Drawable for Cube {
    /// Init a new cube so it's ready to draw.
    fn init(&mut self) {
        let mut index: usize = 0;
        let mut increment = || -> usize {
            let result = index;
            index += 1;
            result
        };
        let mut normal_index: usize = 0;
        let mut normal_increment = || -> usize {
            let normal_result = normal_index;
            normal_index += 1;
            normal_result
        };
        let scale: f32 = self.scale;
        let floor: f32 = self.floor;

        // Bottom
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;

        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        // Left
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;

        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        // Right
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;

        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;

        // Front
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;

        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        // Back
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;

        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        // Top
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;

        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = scale;
        self.vertices[increment()] = floor;

        self.vertices_count = self.vertices.len() as u16;

        // Bottom - UPDATE TO NORMALS FROM VERTS
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;

        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        // Left
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;

        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        // Right
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;

        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;

        // Front
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;

        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = -1.0;
        // Back
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;

        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        // Top
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;

        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 0.0;
        self.normals[normal_increment()] = 1.0;
        self.normals[normal_increment()] = 0.0;
    }

    /// A cube always has the same number of vertices
    fn count_vertices(&self) -> u16 {
        self.vertices_count
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
    fn vertices(&self) -> &[f32] {
        &self.vertices
    }

    /// Get an array of normals.
    fn normals(&self) -> &[f32] {
        &self.normals
    }

    /// Calculate the distance between the cube and the camera.
    fn depth(&self, camera: [f32; 3]) -> f32 {
        ((self.translation[0] - camera[0]).powi(2)
            + (self.translation[1] - camera[1]).powi(2)
            + (self.translation[2] - camera[2]).powi(2))
        .sqrt()
    }
}
