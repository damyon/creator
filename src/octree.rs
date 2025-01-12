


pub mod octree {
    use crate::{cube::cube::Cube, drawable::drawable::Drawable};
    use serde::{Serialize, Deserialize};
    use serde_json;

    pub struct OcTree {
        root: OcNode,
        depth: u32
    }

    impl OcTree {
        pub const fn new() -> OcTree {
            OcTree {
                root: OcNode {
                    x_i: 0, 
                    y_i: 0,
                    z_i: 0,
                    level: 1,
                    active: false,
                    fry: [None, None, None, None, None, None, None, None],
                    has_fry: false,
                    clr: [0.2, 0.2, 0.2, 0.8]
                },
                depth: 1,
            }
        }

        pub fn init(&mut self) {
            
            self.decimate(5);
            
            let saved_value = Self::local_storage().get_item(&"creator_model").unwrap();
            
            if saved_value.is_some() {
                let str = saved_value.unwrap();
                let unflattened: OcNode = serde_json::from_str(&str).unwrap();
             
                self.root = unflattened;
            }
        }

        pub fn drawables(&mut self) -> Vec<Cube> {
            self.root.drawables()
        }

        pub fn decimate(&mut self, levels: u32) {
            self.depth = levels;
            self.root.decimate(levels);
        }

        pub fn local_storage() -> web_sys::Storage {

            let window = web_sys::window();
            window.unwrap().local_storage().unwrap().unwrap()
        }

        pub fn toggle_voxel(&mut self, position: [i32; 3], value: bool, clr: [f32; 4]) {
            self.root.toggle_voxel(position, value, clr);
        }

        pub fn save(&self) {
            let flattened = serde_json::to_string(&self.root).unwrap();
            log::info!("Saving bits");
            let result = Self::local_storage().set_item(&"creator_model", &flattened);
            log::info!("Saved {:?}", result);
        }

        pub fn all_voxels_active(&self, positions: &Vec<[i32; 3]>) -> bool {
            self.root.all_voxels_active(positions)
        }

    }

    #[derive(Serialize, Deserialize)]
    pub struct OcNode {
        x_i: i32,
        y_i: i32,
        z_i: i32,
        level: u32,
        active: bool,
        fry: [Option<Box<Self>>; 8],
        has_fry: bool,
        clr: [f32; 4]
    }

    impl OcNode {

        pub fn decimate(&mut self, levels: u32) {
            if levels > 0 {
                self.subdivide();
        
                let squirts = self.fry.each_mut();

                for index in 0..8 {
                    match squirts[index] {
                        None => {
                            log::debug!("Should not get here")
                        },
                        Some(node) => {
                            node.decimate(levels - 1);
                        }
                    };
                }
            }
        }

        pub fn all_voxels_active(&self, positions: &Vec<[i32; 3]>) -> bool {
            for position in positions {
                if self.x_i == position[0] && self.y_i == position[1] && self.z_i == position[2] && !self.active {
                    return false;
                }
            }
            let squirts = self.fry.each_ref();

            for index in 0..8 {
                match squirts[index] {
                    None => {},
                    Some(node) => {
                        if !node.all_voxels_active(positions) {
                            return false;
                        }
                    }
                };
            }

            return true;
        }

        pub fn toggle_voxel(&mut self, position: [i32; 3], value: bool, clr: [f32; 4]) {

            if self.x_i == position[0] && self.y_i == position[1] && self.z_i == position[2] {
                self.active = value;
                self.clr = clr;
            }
            let squirts = self.fry.each_mut();

            for index in 0..8 {
                match squirts[index] {
                    None => {},
                    Some(node) => {
                        node.toggle_voxel(position, value, clr);
                    }
                };
            }


        }

        pub fn drawables(&mut self) -> Vec<Cube> {
            if self.has_fry {
                let mut child_cubes: Vec<Cube> = vec![];

                let squirts = self.fry.each_mut();

                for index in 0..8 {
                    match squirts[index] {
                        None => { },
                        Some(node) => {
                            let mut cube = node.drawables();

                            child_cubes.append(&mut cube);
                        }
                    };
                }

                child_cubes
            } else {
                if self.active {
                    let scale = 1.0;
                    let mut cube = Cube::new();

                    cube.color = self.clr;
                    cube.scale = scale;
                    cube.init();

                    let x = self.x_i as f32 * (scale);
                    let y = self.y_i as f32 * (scale);
                    let z = self.z_i as f32 * (scale);

                    cube.translate([x, y, z]);

                    vec![cube]
                } else {
                    vec![]
                }
            }
        }

        pub fn subdivide(&mut self) {
            self.has_fry = true;
            let active = false;

            if self.level < 2 {
                self.fry[0] = Some(
                    Box::new(OcNode {
                        x_i: -1,
                        y_i: -1,
                        z_i: -1,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );

                self.fry[1] = Some(
                    Box::new(OcNode {
                        x_i: 0,
                        y_i: -1,
                        z_i: -1,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[2] = Some(
                    Box::new(OcNode {
                        x_i: -1,
                        y_i: 0,
                        z_i: -1,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[3] = Some(
                    Box::new(OcNode {
                        x_i: -1,
                        y_i: -1,
                        z_i: 0,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[4] = Some(
                    Box::new(OcNode {
                        x_i: 0,
                        y_i: 0,
                        z_i: -1,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[5] = Some(
                    Box::new(OcNode {
                        x_i: -1,
                        y_i: 0,
                        z_i: 0,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[6] = Some(
                    Box::new(OcNode {
                        x_i: 0,
                        y_i: -1,
                        z_i: 0,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[7] = Some(
                    Box::new(OcNode {
                        x_i: 0,
                        y_i: 0,
                        z_i: 0,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
            } else {
                
                self.fry[0] = Some(
                    Box::new(OcNode {
                        x_i: self.x_i * 2,
                        y_i: self.y_i * 2,
                        z_i: self.z_i * 2,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[1] = Some(
                    Box::new(OcNode {
                        x_i: self.x_i * 2 + 1,
                        y_i: self.y_i * 2,
                        z_i: self.z_i * 2,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[2] = Some(
                    Box::new(OcNode {
                        x_i: self.x_i * 2,
                        y_i: self.y_i * 2 + 1,
                        z_i: self.z_i * 2,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[3] = Some(
                    Box::new(OcNode {
                        x_i: self.x_i * 2,
                        y_i: self.y_i * 2,
                        z_i: self.z_i * 2 + 1,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[4] = Some(
                    Box::new(OcNode {
                        x_i: self.x_i * 2 + 1,
                        y_i: self.y_i * 2 + 1,
                        z_i: self.z_i * 2,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[5] = Some(
                    Box::new(OcNode {
                        x_i: self.x_i * 2,
                        y_i: self.y_i * 2 + 1,
                        z_i: self.z_i * 2 + 1,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[6] = Some(
                    Box::new(OcNode {
                        x_i: self.x_i * 2 + 1,
                        y_i: self.y_i * 2,
                        z_i: self.z_i * 2 + 1,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
                self.fry[7] = Some(
                    Box::new(OcNode {
                        x_i: self.x_i * 2 + 1,
                        y_i: self.y_i * 2 + 1,
                        z_i: self.z_i * 2 + 1,
                        level: self.level + 1,
                        active: active,
                        fry: [None, None, None, None, None, None, None, None],
                        has_fry: false,
                        clr: self.clr
                    })
                );
            }
         }
    }
}