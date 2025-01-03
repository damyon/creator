


pub mod octree {

    pub struct OcTree {
        children: Option<[Box<OcNode>; 8]>
    }

    impl OcTree {
        pub const fn new() -> OcTree {
            OcTree {
                children: None
            }
        }
    }

    pub struct OcNode {
        x_index: u32,
        y_index: u32,
        z_index: u32,
        subdivide_level: u32,
        active: bool,
        children: [Option<Box<Self>>; 8]
    }

    impl OcNode {
        pub const fn new() -> OcNode {
            OcNode {
                x_index: 0, 
                y_index: 0,
                z_index: 0,
                subdivide_level: 1,
                active: false,
                children: [None, None, None, None, None, None, None, None]
            }
        }

        pub fn decimate(&mut self, levels: u32) {
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

        pub fn subdivide(&mut self) {
            self.children[0] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2, 
                    y_index: self.y_index * 2,
                    z_index: self.z_index * 2,
                    subdivide_level: self.subdivide_level + 1,
                    active: false,
                    children: [None, None, None, None, None, None, None, None]
                })
            );

            self.children[1] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2 + 1, 
                    y_index: self.y_index * 2,
                    z_index: self.z_index * 2,
                    subdivide_level: self.subdivide_level + 1,
                    active: false,
                    children: [None, None, None, None, None, None, None, None]
                })
            );
    
            self.children[2] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2, 
                    y_index: self.y_index * 2 + 1,
                    z_index: self.z_index * 2,
                    subdivide_level: self.subdivide_level + 1,
                    active: false,
                    children: [None, None, None, None, None, None, None, None]
                })
            );
            self.children[3] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2, 
                    y_index: self.y_index * 2,
                    z_index: self.z_index * 2 + 1,
                    subdivide_level: self.subdivide_level + 1,
                    active: false,
                    children: [None, None, None, None, None, None, None, None]
                })
            );
            self.children[4] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2 + 1, 
                    y_index: self.y_index * 2 + 1,
                    z_index: self.z_index * 2,
                    subdivide_level: self.subdivide_level + 1,
                    active: false,
                    children: [None, None, None, None, None, None, None, None]
                })
            );
            self.children[5] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2, 
                    y_index: self.y_index * 2 + 1,
                    z_index: self.z_index * 2 + 1,
                    subdivide_level: self.subdivide_level + 1,
                    active: false,
                    children: [None, None, None, None, None, None, None, None]
                })
            );
            self.children[6] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2 + 1, 
                    y_index: self.y_index * 2,
                    z_index: self.z_index * 2 + 1,
                    subdivide_level: self.subdivide_level + 1,
                    active: false,
                    children: [None, None, None, None, None, None, None, None]
                })
            );
            self.children[7] = Some(
                Box::new(OcNode {
                    x_index: self.x_index * 2 + 1, 
                    y_index: self.y_index * 2 + 1,
                    z_index: self.z_index * 2 + 1,
                    subdivide_level: self.subdivide_level + 1,
                    active: false,
                    children: [None, None, None, None, None, None, None, None]
                })
            );
         }
    }
}