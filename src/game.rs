use std::boxed::Box;

use crate::render_target::RenderTarget;
use crate::vertex_buffer::VertexBuffer;
use crate::mesh::Mesh;
use crate::shader::Shader;

use crate::block_manager::BlockManager;
use crate::block::Block;
use crate::actor::Actor;
use crate::test_actor::TestActor;

type Vec3 = cgmath::Vector3<f32>;
type Vec2 = cgmath::Vector2<f32>;
type IVec2 = cgmath::Vector2<u32>;

pub struct Game {
    actors: Vec<Box<dyn Actor>>,
    g_buffer: RenderTarget,
    quad: VertexBuffer,
    def: Shader
}

impl Game {
    pub fn new() -> Game {
        let g_buffer = RenderTarget::new(1280, 720, 3, false);

        let quad = VertexBuffer::from_mesh(Mesh {
            positions: Some(vec![Vec3::new(-1.0, -1.0, 0.0),
                            Vec3::new(1.0, -1.0, 0.0),
                            Vec3::new(-1.0, 1.0, 0.0),
                            Vec3::new(-1.0, 1.0, 0.0),
                            Vec3::new(1.0, -1.0, 0.0),
                            Vec3::new(1.0, 1.0, 0.0)]),
            normals: None,
            tex_coords: Some(vec![Vec2::new(0.0, 0.0),
                            Vec2::new(1.0, 0.0),
                            Vec2::new(0.0, 1.0),
                            Vec2::new(0.0, 1.0),
                            Vec2::new(1.0, 0.0),
                            Vec2::new(1.0, 1.0)])
        });

        let def = Shader::new(vec![Box::new((gl::FRAGMENT_SHADER, "res/shaders/def.fs.glsl".to_string())),
                                Box::new((gl::VERTEX_SHADER, "res/shaders/def.vs.glsl".to_string()))]);

        let mut manager = BlockManager::new();

        manager.add_block(0, Block::new("Air", false, vec![]));
        manager.add_block(1, Block::new("Stone", false, vec![IVec2::new(1, 0)]));
        manager.add_block(2, Block::new("Dirt", false, vec![IVec2::new(2, 0)]));

        let a = TestActor::new(&manager);
        Game { actors: vec![Box::new(a)], g_buffer: g_buffer, quad: quad, def: def.unwrap() }
    }

    pub fn render(&self) {
        self.g_buffer.bind(); check_gl!();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        for actor in self.actors.iter() {
            actor.render();
        }

        RenderTarget::reset(); check_gl!();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.def.bind(); check_gl!();
        self.def.uniform_texture("_Albedo".to_string(), &self.g_buffer.color_attachments[0], 0); check_gl!();
        self.def.uniform_texture("_Normal".to_string(), &self.g_buffer.color_attachments[1], 1); 
        self.def.uniform_texture("_Position".to_string(), &self.g_buffer.color_attachments[2], 2); 
        self.quad.render();
    }

    pub fn update(&self) {
        for actor in self.actors.iter() {
            actor.update();
        }
    }
}
