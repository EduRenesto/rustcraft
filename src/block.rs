type IVec2 = cgmath::Vector2<u32>;
type Vec2 = cgmath::Vector2<f32>;

pub static TERRAIN_WIDTH: usize = 16;
pub static TERRAIN_HEIGHT: usize = 16;

pub struct Block {
    pub orientable: bool,

    // this holds the position of the textures in the terrain.png
    // 0: +x
    // 1: -x
    // 2: +y
    // 3: -y
    // 4: +z
    // 5: -z
    pub textures: Vec<IVec2>,

    pub name: &'static str
}

impl Block {
    pub fn new(name: &'static str, orientable: bool, textures: Vec<IVec2>) -> Block {
        Block {
            orientable: orientable,
            textures: textures,
            name: name
        }
    }

    pub  fn get_uvs(&self) -> Vec<Vec2> {
        let mut uvs = Vec::<Vec2>::new();

        for tex in self.textures.iter() {
            let u = (1.0 / TERRAIN_WIDTH as f32) * (tex.x as f32);
            let v = 1.0 - ((1.0 / TERRAIN_HEIGHT as f32) * (tex.y as f32 + 1.0));

            uvs.push(Vec2::new(u, v));
        }

        uvs
    }
}
