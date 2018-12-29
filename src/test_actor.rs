use cgmath::{Vector2, Vector3};

use crate::actor::Actor;
use crate::mesh::Mesh;
use crate::vertex_buffer::VertexBuffer;

pub struct TestActor {
    vbo: VertexBuffer
}

impl TestActor {
    pub fn new() -> TestActor {
        let m = Mesh {
            positions: Some(vec![Vector3::<f32>::new(-0.5, -0.5, 0.0),
                                Vector3::<f32>::new(0.0, 0.5, 0.0),
                                Vector3::<f32>::new(0.5, -0.5, 0.0)]),
            normals: Some(vec![Vector3::<f32>::new(-0.5, -0.5, 0.0),
                                Vector3::<f32>::new(-0.5, -0.5, 0.0),
                                Vector3::<f32>::new(-0.5, -0.5, 0.0)]),
            tex_coords: Some(vec![Vector2::<f32>::new(0.0, 0.0),
                                Vector2::<f32>::new(0.5, 1.0),
                                Vector2::<f32>::new(1.0, 0.0)])
        };

        TestActor { vbo: VertexBuffer::from_mesh(m) }
    }
}

impl Actor for TestActor {
    fn render(&self) {
        self.vbo.render();
    }

    fn update(&mut self) {
    }
}
