use std::cell::RefCell;

use crate::chunk::Chunk;

use noise::{NoiseFn, OpenSimplex, Seedable};

type Vec3 = cgmath::Vector3<f32>;
type IVec3 = cgmath::Vector3<i32>;

pub struct World {
   pub chunks: Vec<Chunk>
}

impl World {
    pub fn gen_world() -> World {
       let mut chunks = Vec::<Chunk>::new(); 

       for x in -2..2 {
           for z in -2..2 {
               chunks.push(World::generate_chunk(IVec3::new(x, -1, z)));
           }
       }

       World { chunks }
    }

    pub fn generate_chunk(position: IVec3) -> Chunk {
        let noise = OpenSimplex::new();
        noise.set_seed(120934834);

        let mut blocks = [[[0; 18]; 66]; 18];

        for x in 0..15 {
            for y in 0..63 {
                for z in 0..15 {
                    let pos = Vec3::new((position.x*16 + x) as f32,
                                        (position.y*64 + y) as f32,
                                        (position.z*16 + z) as f32) / 10.0;

                    let val = noise.get([pos.x as f64, pos.y as f64, pos.z as f64]);

                    if val <= 0.0 {
                        blocks[(x+1) as usize][(y+1) as usize][(z+1) as usize] = 2;
                    }
                }
            }
        }

        Chunk {
            blocks: blocks,
            position: position
        }
    }
}