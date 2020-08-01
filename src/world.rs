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

        let mut blocks = [[[0; 18]; 128]; 18];

        for x in 0..17 {
            for y in 0..127 {
                for z in 0..17 {
                    let pos = Vec3::new((position.x*16 + x) as f32,
                                        (position.y*128 + y) as f32,
                                        (position.z*16 + z) as f32) / 30.0;

                    let map = (1..=3).into_iter()
                        .map(|i| (i as f32, pos/i as f32))
                        .map(|(i, v)| i * 10.0 * noise.get([v.x as f64, v.y as f64, v.z as f64]) as f32)
                        .sum::<f32>();

                    let val = ( y as f32 - 64.0 ) + map;

                    if val <= 0.0 {
                        //let block = if 0 <= y && y <= 20 {
                            //1
                        //} else if y > 20 && y <= 30 {
                            //3
                        //} else if y == 50 {
                            //4
                        //} else {
                            //0
                        //};

                        let block = 1;

                        blocks[x as usize][y as usize][z as usize] = block;
                    } else {
                        blocks[x as usize][y as usize][z as usize] = 0;
                    }
                }
            }
        }


        Chunk {
            blocks,
            position
        }
    }
}
