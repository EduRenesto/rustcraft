use crate::mesh::Mesh;

use crate::block_manager::BlockManager;
use crate::block::{TERRAIN_WIDTH, TERRAIN_HEIGHT};

use cgmath::ElementWise;

// just to avoid typing the cgmath thing way too much
type Vec3 = cgmath::Vector3<f32>;
type Vec2 = cgmath::Vector2<f32>;
type IVec3 = cgmath::Vector3<i32>;

// A chunk is a block of 16x16x64 blocks.
// we add 2 to each dimension because of the edges
pub struct Chunk {
    // the blocks
    pub blocks: [[[u32; 16]; 64]; 16], 

    // the position (in "chunk space")
    pub position: IVec3
}

impl Chunk {
    fn process_block(pos: IVec3) {

    }

    pub fn gen_mesh(&self, manager: &BlockManager) -> Mesh {
        let at = |x: usize, y: usize, z: usize| self.blocks[x][y][z];

        let mut positions = Vec::<Vec3>::new();
        let mut normals = Vec::<Vec3>::new();
        let mut uvs = Vec::<Vec2>::new();
        let mut occlusion = Vec::<f32>::new();

        for x in 0..15 {
            for y in 0..63 {
                for z in 0..15 {
                    let block = at(x, y, z);

                    if block == 0 {
                        continue; 
                    }

                    let the_block = manager.get_block(&block).unwrap();
                    let uv_offset = the_block.get_uvs();
                    let uv_scale = Vec2::new(TERRAIN_WIDTH as f32, TERRAIN_HEIGHT as f32);

                    // Calculate the immediate neighbors
                    let pos_x = if x < 15 {
                        at(x + 1, y, z)
                    } else {
                        0 // dont render the chunk borders!
                    };
                    let neg_x = if x > 0 {
                        at(x - 1, y, z)
                    } else {
                        0
                    };

                    let pos_y = if y < 63 {
                        at(x, y + 1, z)
                    } else {
                        0
                    };
                    let neg_y = if y > 0 { 
                        at(x, y - 1, z)
                    } else {
                        0 // always draw and bottom
                    };

                    let pos_z = if z < 15 {
                        at(x, y, z + 1)
                    } else {
                        0
                    };
                    let neg_z = if z > 0 {
                        at(x, y, z - 1)
                    } else {
                        0
                    };

                    // Calculate the "indirect" neighbors
                    // This is used for the occlusion approximation
                    let pos_x_pos_y = if x < 15 && y < 63 {
                        at(x + 1, y + 1, z)
                    } else {
                        // TODO fix this
                        // This will generate incorrect occlusion values
                        // for chunk borders :(
                        0
                    };
                    let pos_x_neg_y = if x < 15 && y > 0 {
                        at(x + 1, y - 1, z)
                    } else {
                        0
                    };
                    let pos_x_pos_z = if x < 15 && z < 15 {
                        at(x + 1, y, z + 1)
                    } else {
                        0
                    };
                    let pos_x_neg_z = if x < 15 && z > 0 {
                        at(x + 1, y, z - 1)
                    } else {
                        0
                    };

                    let neg_x_pos_y = if x > 0 && y < 63 {
                        at(x - 1, y + 1, z)
                    } else {
                        0
                    };
                    let neg_x_neg_y = if x > 0 && y > 0 {
                        at(x - 1, y - 1, z)
                    } else {
                        0
                    };
                    let neg_x_pos_z = if x > 0 && z < 15 {
                        at(x - 1, y, z + 1)
                    } else {
                        0
                    };
                    let neg_x_neg_z = if x > 0 && z > 0 {
                        at(x - 1, y, z - 1)
                    } else {
                        0
                    };

                    let pos_y_pos_z = if y < 63 && z < 15 {
                        at(x, y + 1, z + 1)
                    } else {
                        0
                    };
                    let pos_y_neg_z = if y < 63 && z > 0 {
                        at(x, y + 1, z - 1)
                    } else {
                        0
                    };
                    
                    let neg_y_pos_z = if y > 0 && z < 15 {
                        at(x, y - 1, z + 1)
                    } else {
                        0
                    };
                    let neg_y_neg_z = if y > 0 && z > 0 {
                        at(x, y - 1, z - 1)
                    } else {
                        0
                    };

                    let neighbors: Vec<u8> = vec![pos_x_pos_y, pos_x_neg_y,
                                        pos_x_pos_z, pos_x_neg_z,
                                        neg_x_pos_y, neg_x_neg_y,
                                        neg_x_pos_z, neg_x_neg_z,
                                        pos_y_pos_z, pos_y_neg_z,
                                        neg_y_pos_z, neg_y_neg_z]
                        .iter().map(|i| {
                            if *i != 0 {
                                1
                            } else {
                                0
                            }
                        }).collect();

                    if pos_x == 0 {
                        // render the +x face
                        
                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 15 + 1 + (x as i32)) as f32,
                            (self.position.y * 63 + (y as i32)) as f32,
                            (self.position.z * 15 + (z as i32)) as f32
                        );

                        // Begin first triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0)); // A
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0)); // B
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0)); // D

                        normals.push(Vec3::new(1.0, 0.0, 0.0));
                        normals.push(Vec3::new(1.0, 0.0, 0.0));
                        normals.push(Vec3::new(1.0, 0.0, 0.0));

                        uvs.push(uv_offset[0] + Vec2::new(0.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[0] + Vec2::new(1.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[0] + Vec2::new(0.0, 1.0).div_element_wise(uv_scale));

                        occlusion.push(vec![1, 3, 11].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![1, 2, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![0, 1, 09].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0)); // B
                        positions.push(pos + Vec3::new(0.0, 1.0, 1.0)); // C
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0)); // D

                        normals.push(Vec3::new(1.0, 0.0, 0.0));
                        normals.push(Vec3::new(1.0, 0.0, 0.0));
                        normals.push(Vec3::new(1.0, 0.0, 0.0));

                        uvs.push(uv_offset[0] + Vec2::new(0.0, 1.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[0] + Vec2::new(1.0, 1.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[0] + Vec2::new(1.0, 0.0).div_element_wise(uv_scale));

                        occlusion.push(vec![1, 2, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![0, 2, 08].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![0, 1, 09].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End second triangle
                    }
                    if neg_x == 0 {
                        // render the -x facE

                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 15 + (x as i32)) as f32,
                            (self.position.y * 63 + (y as i32)) as f32,
                            (self.position.z * 15 + (z as i32)) as f32
                        );

                        let idx = if the_block.orientable {
                            1
                        } else {
                            0
                        };

                        // Begin first triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0)); // F
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0)); // E
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0)); // G

                        normals.push(Vec3::new(-1.0, 0.0, 0.0));
                        normals.push(Vec3::new(-1.0, 0.0, 0.0));
                        normals.push(Vec3::new(-1.0, 0.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 1.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 0.0).div_element_wise(uv_scale));

                        occlusion.push(vec![5, 7, 11].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![4, 7, 9].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![5, 6, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0)); // G
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0)); // E
                        positions.push(pos + Vec3::new(0.0, 1.0, 1.0)); // H

                        normals.push(Vec3::new(-1.0, 0.0, 0.0));
                        normals.push(Vec3::new(-1.0, 0.0, 0.0));
                        normals.push(Vec3::new(-1.0, 0.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 1.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 0.0).div_element_wise(uv_scale));

                        occlusion.push(vec![5, 6, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![4, 7, 9].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![4, 8, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End second triangle
                    }
                    if pos_y == 0 { 
                        // render the +y face

                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 15 + (x as i32)) as f32,
                            (self.position.y * 63 + 1 + (y as i32)) as f32,
                            (self.position.z * 15 + (z as i32)) as f32
                        );

                        let idx = if the_block.orientable {
                            2
                        } else {
                            0
                        };

                        // Begin first triangle
                        //positions.push(pos + Vec3::new(0.0, 0.0, 0.0));
                        //positions.push(pos + Vec3::new(0.0, 0.0, 1.0));
                        //positions.push(pos + Vec3::new(1.0, 0.0, 0.0));
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0)); // E
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0)); // D
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0)); // H

                        normals.push(Vec3::new(0.0, 1.0, 0.0));
                        normals.push(Vec3::new(0.0, 1.0, 0.0));
                        normals.push(Vec3::new(0.0, 1.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 1.0).div_element_wise(uv_scale));

                        occlusion.push(vec![4, 7, 9].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![0, 1, 9].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![4, 8, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0)); // D
                        positions.push(pos + Vec3::new(1.0, 0.0, 1.0)); // C
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0)); // H

                        normals.push(Vec3::new(0.0, 1.0, 0.0));
                        normals.push(Vec3::new(0.0, 1.0, 0.0));
                        normals.push(Vec3::new(0.0, 1.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 1.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 1.0).div_element_wise(uv_scale));

                        occlusion.push(vec![0, 1, 9].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![0, 2, 8].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![4, 8, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End second triangle
                    }
                    if neg_y == 0 {
                        // render the -y face

                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 15 + (x as i32)) as f32,
                            (self.position.y * 63 + (y as i32)) as f32,
                            (self.position.z * 15 + (z as i32)) as f32
                        );

                        let idx = if the_block.orientable {
                            3
                        } else {
                            0
                        };

                        // Begin first triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0)); // F
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0)); // G
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0)); // A

                        normals.push(Vec3::new(0.0, -1.0, 0.0));
                        normals.push(Vec3::new(0.0, -1.0, 0.0));
                        normals.push(Vec3::new(0.0, -1.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 1.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 0.0).div_element_wise(uv_scale));

                        occlusion.push(vec![5, 7, 11].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![5, 6, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![1, 3, 11].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0)); // A
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0)); // G
                        positions.push(pos + Vec3::new(1.0, 0.0, 1.0)); // B

                        normals.push(Vec3::new(0.0, -1.0, 0.0));
                        normals.push(Vec3::new(0.0, -1.0, 0.0));
                        normals.push(Vec3::new(0.0, -1.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 1.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 0.0).div_element_wise(uv_scale));

                        occlusion.push(vec![1, 3, 11].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![5, 6, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![1, 2, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End second triangle
                    }
                    if pos_z == 0 {
                        // render the +z face

                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 15 + (x as i32)) as f32,
                            (self.position.y * 63 + (y as i32)) as f32,
                            (self.position.z * 15 + 1 + (z as i32)) as f32
                        );

                        let idx = if the_block.orientable {
                            4
                        } else {
                            0
                        };

                        // Begin first triangle
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0)); // B
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0)); // G
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0)); // H

                        normals.push(Vec3::new(0.0, 0.0, 1.0));
                        normals.push(Vec3::new(0.0, 0.0, 1.0));
                        normals.push(Vec3::new(0.0, 0.0, 1.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 1.0).div_element_wise(uv_scale));

                        occlusion.push(vec![1, 2, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![5, 6, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![4, 8, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0)); // H
                        positions.push(pos + Vec3::new(1.0, 1.0, 0.0)); // C
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0)); // B

                        normals.push(Vec3::new(0.0, 0.0, 1.0));
                        normals.push(Vec3::new(0.0, 0.0, 1.0));
                        normals.push(Vec3::new(0.0, 0.0, 1.0));

                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 1.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 1.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 0.0).div_element_wise(uv_scale));

                        occlusion.push(vec![4, 8, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![0, 2, 08].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![1, 2, 10].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End second triangle
                    }
                    if neg_z == 0 {
                        // render the -z face

                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 15 + (x as i32)) as f32,
                            (self.position.y * 63 + (y as i32)) as f32,
                            (self.position.z * 15 + (z as i32)) as f32
                        );

                        let idx = if the_block.orientable {
                            5
                        } else {
                            0
                        };

                        // Begin first triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0)); // F
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0)); // A
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0)); // E

                        normals.push(Vec3::new(0.0, 0.0, -1.0));
                        normals.push(Vec3::new(0.0, 0.0, -1.0));
                        normals.push(Vec3::new(0.0, 0.0, -1.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 1.0).div_element_wise(uv_scale));

                        occlusion.push(vec![5, 7, 11].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![1, 3, 11].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![4, 7, 09].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0)); // E
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0)); // A
                        positions.push(pos + Vec3::new(1.0, 1.0, 0.0)); // D

                        normals.push(Vec3::new(0.0, 0.0, -1.0));
                        normals.push(Vec3::new(0.0, 0.0, -1.0));
                        normals.push(Vec3::new(0.0, 0.0, -1.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0, 1.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 0.0).div_element_wise(uv_scale));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0, 1.0).div_element_wise(uv_scale));

                        occlusion.push(vec![4, 7, 09].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![1, 3, 11].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        occlusion.push(vec![0, 1, 09].iter().map(|i| neighbors[*i] as f32 / 3.0).sum());
                        // End second triangle
                    }
                }
            }
        }

        Mesh { 
            positions: Some(positions), 
            normals: Some(normals), 
            tex_coords: Some(uvs),
            occlusion: Some(occlusion)
        }
    }
}
