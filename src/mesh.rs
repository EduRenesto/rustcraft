use cgmath::Vector3;
use cgmath::Vector2;

pub struct Mesh {
    pub positions: Option<Vec<Vector3<f32>>>,
    pub normals: Option<Vec<Vector3<f32>>>,
    pub tex_coords: Option<Vec<Vector2<f32>>>,
    pub occlusion: Option<Vec<f32>>
}

impl Default for Mesh {
    fn default() -> Mesh {
        Mesh {
            positions: None,
            normals: None,
            tex_coords: None,
            occlusion: None
        }
    } 
}
