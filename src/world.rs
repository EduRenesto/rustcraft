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

       for x in -5..5 {
           for z in -5..5 {
               chunks.push(World::generate_chunk(IVec3::new(x, -1, z)));
           }
       }

       World { chunks }
    }

    pub fn generate_chunk(position: IVec3) -> Chunk {
        let noise = OpenSimplex::new();
        //noise.set_seed(120934834);
        noise.set_seed(1831823);

        let mut blocks = [[[0; 18]; 64]; 18];

        for x in 0..17 {
            for y in 0..63 {
                for z in 0..17 {
                    let pos = Vec3::new((position.x*16 + x) as f32,
                                        (position.y*64 + y) as f32,
                                        (position.z*16 + z) as f32) / 30.0;
                    let octave = Vec3::new((position.x*16 + x) as f32,
                                        (position.y*64 + y) as f32,
                                        (position.z*16 + z) as f32) / 60.0;

                    let val = noise.get([pos.x as f64, pos.y as f64, pos.z as f64]) +
                              noise.get([octave.x as f64, octave.y as f64, octave.z as f64]);

                    if val <= 0.0 {
                        let block = if 0 <= y && y <= 20 {
                            1
                        } else if y > 20 && y <= 30 {
                            3
                        } else if y == 50 {
                            4
                        } else {
                            0
                        };

                        blocks[(x) as usize][(y) as usize][(z) as usize] = block;
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
