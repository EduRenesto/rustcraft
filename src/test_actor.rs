use std::cell::Cell;
use std::cell::Ref;

use cgmath::Vector3;
use cgmath::Matrix4;
use cgmath::Point3;

use crate::actor::Actor;
use crate::camera::Camera;
use crate::vertex_buffer::VertexBuffer;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::block_manager::BlockManager;
use crate::world::World;

use gl::{FRAGMENT_SHADER, VERTEX_SHADER};

pub struct TestActor {
    vbos: Vec<VertexBuffer>,
    shader: Shader,
    proj_matrix: Matrix4<f32>,
    text: Texture,
    time: Cell<f32>
}

impl TestActor {
    pub fn new(manager: &BlockManager) -> TestActor {
        let chunks = World::gen_world().chunks;

        let m = chunks.iter().map(|c| VertexBuffer::from_mesh(c.gen_mesh(manager)));

        let shader = Shader::new(vec![Box::new((VERTEX_SHADER, "res/shaders/simple.vs.glsl".to_string())),
                                    Box::new((FRAGMENT_SHADER, "res/shaders/simple.fs.glsl".to_string()))])
            .unwrap();

        let text = Texture::from_file("res/textures/terrain.png".to_string(),
                                               gl::NEAREST_MIPMAP_NEAREST as i32,
                                               gl::NEAREST as i32)
            .unwrap();
        text.gen_mipmaps();

        TestActor { 
            vbos: m.collect(), 
            shader: shader,
            proj_matrix: cgmath::perspective(cgmath::Deg(60.0), 16.0/9.0, 0.01, 1000.0),
            text: text,
            time: Cell::new(0.0)
        }
    }
}

impl Actor for TestActor {
    fn render(&self, camera: &dyn Camera) {
        self.shader.bind();
        self.text.bind();
        self.shader.uniform_texture("_Text".to_string(), &self.text, 0);
        self.shader.uniform_mat4x4("_Projection".to_string(), self.proj_matrix);
        self.shader.uniform_mat4x4("_View".to_string(), camera.get_view_matrix());
        self.shader.uniform_float32("_Time".to_string(), self.time.get());

        for vbo in self.vbos.iter() {
            vbo.render();
        }
    }

    fn update(&self) {
        let val = self.time.get();
        self.time.set(val + 0.01);
    }
}
