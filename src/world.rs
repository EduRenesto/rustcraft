use std::cell::RefCell;

use crate::chunk::Chunk;

use noise::{NoiseFn, OpenSimplex, Seedable};

type Vec3 = cgmath::Vector3<f32>;
type IVec3 = cgmath::Vector3<i32>;

pub struct World {
   chunks: Vec<Chunk>
}

impl World {
    pub fn generate_chunk() -> Chunk {
        let noise = OpenSimplex::new();
        noise.set_seed(120934834);

        let mut blocks = [[[0; 18]; 66]; 18];

        for x in 0..15 {
            for y in 0..63 {
                for z in 0..15 {
                    let val = noise.get([x as f64 / 10.0, y as f64 / 10.0, z as f64 / 10.0]);

                    if val <= 0.0 {
                        blocks[x+1][y+1][z+1] = 2;
                    }
                }
            }
        }

        Chunk {
            blocks: blocks,
            position: IVec3::new(0, -1, 0)
        }
    }
}
