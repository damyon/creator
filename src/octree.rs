


pub mod octree {
    use crate::{cube::cube::Cube, drawable::drawable::Drawable};


    pub struct OcTree {
        root: OcNode,
        depth: u32
    }

    impl OcTree {
        pub const fn new() -> OcTree {
            OcTree {
                root: OcNode {
                    x_index: 0, 
                    y_index: 0,
                    z_index: 0,
                    subdivide_level: 1,
                    active: false,
                    children: [None, None, None, None, None, None, None, None],
                    has_children: false
                },
                depth: 1
            }
        }

        pub fn init(&mut self) {
            self.decimate(3);
        }

        pub fn drawables(&mut self) -> Vec<Cube> {
            self.root.drawables()
        }

        pub fn decimate(&mut self, levels: u32) {
            log::info!("We are decimating {}", levels);
            self.depth = levels;
            self.root.decimate(levels);
        }

        pub fn toggle_voxel(&mut self, position: [u32; 3]) {
            self.root.toggle_voxel(position);
        }

    }

    pub struct OcNode {
        x_index: u32,
        y_index: u32,
        z_index: u32,
        subdivide_level: u32,
        active: bool,
        children: [Option<Box<Self>>; 8],
        has_children: bool
    }

    impl OcNode {

        pub fn decimate(&mut self, levels: u32) {
            log::info!("We are decimating {}", levels);
            if levels > 0 {
                self.subdivide();
        
                let squirts = self.children.each_mut();

                for index in 0..8 {
                    match squirts[index] {
                        None => {
                            log::info!("Should not get here")
                        },
                        Some(node) => {
                            node.decimate(levels - 1);
                        }
                    };
                }
            }
        }

        pub fn toggle_voxel(&mut self, position: [u32; 3]) {
            if self.x_index-4 == position[0] && self.y_index-4 == position[1] && self.z_index-4 == position[2] {
                self.active = !self.active;
            }
            let squirts = self.children.each_mut();

            for index in 0..8 {
                match squirts[index] {
                    None => {
                        log::info!("Should not get here")
                    },
                    Some(node) => {
                        node.toggle_voxel(position);
                    }
                };
            }
        }

        pub fn drawables(&mut self) -> Vec<Cube> {
            if self.has_children {
                let mut child_cubes: Vec<Cube> = vec![];

                let squirts = self.children.each_mut();

                for index in 0..8 {
                    match squirts[index] {
                        None => {
                            log::info!("Should not get here")
                        },
                        Some(node) => {
                            let mut cube = node.drawables();

                            child_cubes.append(&mut cube);
                        }
                    };
                }

                child_cubes
            } else {
                if self.active {
                    let scale = 5.0 / (self.subdivide_level as f32 + 1.0);
                    let mut cube = Cube::new();
                    cube.color = [0.4, 0.4, 0.2, 0.4];
                    cube.scale = scale;
                    cube.init();

                    let x = self.x_index as f32 * scale - 4.0;
                    let y = self.y_index as f32 * scale - 4.0;
                    let z = self.z_index as f32 * scale - 4.0;

                    cube.translate([x, y, z]);

                    vec![cube]
                } else {
                    vec![]
                }
            }
        }

        pub fn subdivide(&mut self) {
            log::info!("We are subdividing");
            self.has_children = true;
            let active = false;
            self.children[0] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2, 
                    y_index: self.y_index * 2,
                    z_index: self.z_index * 2,
                    subdivide_level: self.subdivide_level + 1,
                    active: active,
                    children: [None, None, None, None, None, None, None, None],
                    has_children: false
                })
            );

            self.children[1] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2 + 1, 
                    y_index: self.y_index * 2,
                    z_index: self.z_index * 2,
                    subdivide_level: self.subdivide_level + 1,
                    active: active,
                    children: [None, None, None, None, None, None, None, None],
                    has_children: false
                })
            );
    
            self.children[2] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2, 
                    y_index: self.y_index * 2 + 1,
                    z_index: self.z_index * 2,
                    subdivide_level: self.subdivide_level + 1,
                    active: active,
                    children: [None, None, None, None, None, None, None, None],
                    has_children: false
                })
            );
            self.children[3] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2, 
                    y_index: self.y_index * 2,
                    z_index: self.z_index * 2 + 1,
                    subdivide_level: self.subdivide_level + 1,
                    active: active,
                    children: [None, None, None, None, None, None, None, None],
                    has_children: false
                })
            );
            self.children[4] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2 + 1, 
                    y_index: self.y_index * 2 + 1,
                    z_index: self.z_index * 2,
                    subdivide_level: self.subdivide_level + 1,
                    active: active,
                    children: [None, None, None, None, None, None, None, None],
                    has_children: false
                })
            );
            self.children[5] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2, 
                    y_index: self.y_index * 2 + 1,
                    z_index: self.z_index * 2 + 1,
                    subdivide_level: self.subdivide_level + 1,
                    active: active,
                    children: [None, None, None, None, None, None, None, None],
                    has_children: false
                })
            );
            self.children[6] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2 + 1, 
                    y_index: self.y_index * 2,
                    z_index: self.z_index * 2 + 1,
                    subdivide_level: self.subdivide_level + 1,
                    active: active,
                    children: [None, None, None, None, None, None, None, None],
                    has_children: false
                })
            );
            self.children[7] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2 + 1, 
                    y_index: self.y_index * 2 + 1,
                    z_index: self.z_index * 2 + 1,
                    subdivide_level: self.subdivide_level + 1,
                    active: active,
                    children: [None, None, None, None, None, None, None, None],
                    has_children: false
                })
            );
         }
    }
}