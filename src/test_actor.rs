use cgmath::{Vector2, Vector3};
use cgmath::Matrix4;

use crate::actor::Actor;
use crate::mesh::Mesh;
use crate::vertex_buffer::VertexBuffer;
use crate::shader::Shader;
use crate::texture::Texture;

use gl::{FRAGMENT_SHADER, VERTEX_SHADER};

pub struct TestActor {
    vbo: VertexBuffer,
    shader: Shader,
    proj_matrix: Matrix4<f32>,
    text: Texture
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

        let shader = Shader::new(vec![Box::new((VERTEX_SHADER, "res/shaders/simple.vs.glsl".to_string())),
                                    Box::new((FRAGMENT_SHADER, "res/shaders/simple.fs.glsl".to_string()))])
            .unwrap();

        let text = Texture::from_file("res/textures/test.png".to_string(),
                                               gl::NEAREST as i32,
                                               gl::NEAREST as i32)
            .unwrap();

        TestActor { 
            vbo: VertexBuffer::from_mesh(m), 
            shader: shader,
            proj_matrix: cgmath::perspective(cgmath::Deg(60.0), 16.0/9.0, 0.00001, 100000.0),
            text: text
        }
    }
}

impl Actor for TestActor {
    fn render(&self) {
        self.shader.bind();
        self.text.bind();
        self.shader.uniform_mat4x4("_Projection".to_string(), self.proj_matrix);
        self.vbo.render();
    }

    fn update(&mut self) {
    }
}
