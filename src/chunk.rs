use crate::mesh::Mesh;

use crate::block_manager::BlockManager;
use crate::block::{TERRAIN_WIDTH, TERRAIN_HEIGHT};

// just to avoid typing the cgmath thing way too much
type Vec3 = cgmath::Vector3<f32>;
type Vec2 = cgmath::Vector2<f32>;
type IVec3 = cgmath::Vector3<i32>;

// A chunk is a block of 16x16x64 blocks.
// we add 2 to each dimension because of the edges
pub struct Chunk {
    // the blocks
    pub blocks: [[[u32; 18]; 66]; 18], 

    // the position (in "chunk space")
    pub position: IVec3
}

impl Chunk {
    pub fn gen_mesh(&self, manager: &BlockManager) -> Mesh {
        let at = |x: usize, y: usize, z: usize| self.blocks[x][y][z];

        let mut positions = Vec::<Vec3>::new();
        let mut normals = Vec::<Vec3>::new();
        let mut uvs = Vec::<Vec2>::new();

        for x in 1..16 {
            for y in 1..64 {
                for z in 1..16 {
                    let block = at(x, y, z);

                    if block == 0 {
                        continue; 
                    }

                    let the_block = manager.get_block(&block).unwrap();
                    let uv_offset = the_block.get_uvs();

                    let pos_x = at(x + 1, y, z);
                    let neg_x = at(x - 1, y, z);
                    let pos_y = at(x, y + 1, z);
                    let neg_y = at(x, y - 1, z);
                    let pos_z = at(x, y, z + 1);
                    let neg_z = at(x, y, z - 1);

                    if pos_x == 0 {
                        // render the +x face
                        
                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 16 + 1 + (x as i32 - 1)) as f32,
                            (self.position.y * 64 + (y as i32 - 1)) as f32,
                            (self.position.z * 16 + (z as i32 - 1)) as f32
                        );

                        // Begin first triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0));
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0));
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0));

                        normals.push(Vec3::new(1.0, 0.0, 0.0));
                        normals.push(Vec3::new(1.0, 0.0, 0.0));
                        normals.push(Vec3::new(1.0, 0.0, 0.0));

                        uvs.push(uv_offset[0] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[0] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[0] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0));
                        positions.push(pos + Vec3::new(0.0, 1.0, 1.0));
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0));

                        normals.push(Vec3::new(1.0, 0.0, 0.0));
                        normals.push(Vec3::new(1.0, 0.0, 0.0));
                        normals.push(Vec3::new(1.0, 0.0, 0.0));

                        uvs.push(uv_offset[0] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[0] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[0] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        // End second triangle
                    }
                    if neg_x == 0 {
                        // render the -x facE

                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 16 + (x as i32 - 1)) as f32,
                            (self.position.y * 64 + (y as i32 - 1)) as f32,
                            (self.position.z * 16 + (z as i32 - 1)) as f32
                        );

                        let idx = if the_block.orientable {
                            1
                        } else {
                            0
                        };

                        // Begin first triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0));
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0));
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0));

                        normals.push(Vec3::new(-1.0, 0.0, 0.0));
                        normals.push(Vec3::new(-1.0, 0.0, 0.0));
                        normals.push(Vec3::new(-1.0, 0.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0));
                        positions.push(pos + Vec3::new(0.0, 1.0, 1.0));
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0));

                        normals.push(Vec3::new(-1.0, 0.0, 0.0));
                        normals.push(Vec3::new(-1.0, 0.0, 0.0));
                        normals.push(Vec3::new(-1.0, 0.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        // End second triangle
                    }
                    if pos_y == 0 { 
                        // render the +y face

                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 16 + (x as i32 - 1)) as f32,
                            (self.position.y * 64 + 1 + (y as i32 - 1)) as f32,
                            (self.position.z * 16 + (z as i32 - 1)) as f32
                        );

                        let idx = if the_block.orientable {
                            2
                        } else {
                            0
                        };

                        // Begin first triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0));
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0));
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0));

                        normals.push(Vec3::new(0.0, 1.0, 0.0));
                        normals.push(Vec3::new(0.0, 1.0, 0.0));
                        normals.push(Vec3::new(0.0, 1.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0));
                        positions.push(pos + Vec3::new(1.0, 0.0, 1.0));
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0));

                        normals.push(Vec3::new(0.0, 1.0, 0.0));
                        normals.push(Vec3::new(0.0, 1.0, 0.0));
                        normals.push(Vec3::new(0.0, 1.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        // End second triangle
                    }
                    if neg_y == 0 {
                        // render the -y face

                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 16 + (x as i32 - 1)) as f32,
                            (self.position.y * 64 + (y as i32 - 1)) as f32,
                            (self.position.z * 16 + (z as i32 - 1)) as f32
                        );

                        let idx = if the_block.orientable {
                            3
                        } else {
                            0
                        };

                        // Begin first triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0));
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0));
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0));

                        normals.push(Vec3::new(0.0, -1.0, 0.0));
                        normals.push(Vec3::new(0.0, -1.0, 0.0));
                        normals.push(Vec3::new(0.0, -1.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0));
                        positions.push(pos + Vec3::new(1.0, 0.0, 1.0));
                        positions.push(pos + Vec3::new(0.0, 0.0, 1.0));

                        normals.push(Vec3::new(0.0, -1.0, 0.0));
                        normals.push(Vec3::new(0.0, -1.0, 0.0));
                        normals.push(Vec3::new(0.0, -1.0, 0.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        // End second triangle
                    }
                    if pos_z == 0 {
                        // render the +z face

                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 16 + (x as i32 - 1)) as f32,
                            (self.position.y * 64 + (y as i32 - 1)) as f32,
                            (self.position.z * 16 + 1 + (z as i32 - 1)) as f32
                        );

                        let idx = if the_block.orientable {
                            4
                        } else {
                            0
                        };

                        // Begin first triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0));
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0));
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0));

                        normals.push(Vec3::new(0.0, 0.0, 1.0));
                        normals.push(Vec3::new(0.0, 0.0, 1.0));
                        normals.push(Vec3::new(0.0, 0.0, 1.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0));
                        positions.push(pos + Vec3::new(1.0, 1.0, 0.0));
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0));

                        normals.push(Vec3::new(0.0, 0.0, 1.0));
                        normals.push(Vec3::new(0.0, 0.0, 1.0));
                        normals.push(Vec3::new(0.0, 0.0, 1.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        // End second triangle
                    }
                    if neg_z == 0 {
                        // render the -z face

                        // position of the face
                        let pos = Vec3::new(
                            (self.position.x * 16 + (x as i32 - 1)) as f32,
                            (self.position.y * 64 + (y as i32 - 1)) as f32,
                            (self.position.z * 16 + (z as i32 - 1)) as f32
                        );

                        let idx = if the_block.orientable {
                            5
                        } else {
                            0
                        };

                        // Begin first triangle
                        positions.push(pos + Vec3::new(0.0, 0.0, 0.0));
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0));
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0));

                        normals.push(Vec3::new(0.0, 0.0, -1.0));
                        normals.push(Vec3::new(0.0, 0.0, -1.0));
                        normals.push(Vec3::new(0.0, 0.0, -1.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        // End first triangle
                        
                        // Begin second triangle
                        positions.push(pos + Vec3::new(1.0, 0.0, 0.0));
                        positions.push(pos + Vec3::new(1.0, 1.0, 0.0));
                        positions.push(pos + Vec3::new(0.0, 1.0, 0.0));

                        normals.push(Vec3::new(0.0, 0.0, -1.0));
                        normals.push(Vec3::new(0.0, 0.0, -1.0));
                        normals.push(Vec3::new(0.0, 0.0, -1.0));

                        uvs.push(uv_offset[idx] + Vec2::new(0.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 1.0 / TERRAIN_HEIGHT as f32));
                        uvs.push(uv_offset[idx] + Vec2::new(1.0 / TERRAIN_WIDTH as f32, 0.0 / TERRAIN_HEIGHT as f32));
                        // End second triangle
                    }
                }
            }
        }

        Mesh { positions: Some(positions), normals: Some(normals), tex_coords: Some(uvs) }
    }
}
