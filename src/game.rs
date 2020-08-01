use std::boxed::Box;
use std::cell::RefCell;

use crate::render_target::RenderTarget;
use crate::vertex_buffer::VertexBuffer;
use crate::mesh::Mesh;
use crate::shader::Shader;
use crate::texture::Texture;
use crate::fps_camera::FpsCamera;

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
    def: Shader,
    camera: RefCell<FpsCamera>,
    gizmo: VertexBuffer,
    gizmo_shader: Shader,
}

impl Game {
    pub fn new() -> Game {
        let g_buffer = RenderTarget::new(1280, 720, 4, false);

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
                            Vec2::new(1.0, 1.0)]),
            occlusion: None
        });

        let def = Shader::new(vec![Box::new((gl::FRAGMENT_SHADER, "res/shaders/def.fs.glsl".to_string())),
                                Box::new((gl::VERTEX_SHADER, "res/shaders/def.vs.glsl".to_string()))]);

        let mut manager = BlockManager::new();

        manager.add_block(0, Block::new("Air", false, vec![]));
        manager.add_block(1, Block::new("Stone", false, vec![IVec2::new(1, 0)]));
        manager.add_block(2, Block::new("Dirt", false, vec![IVec2::new(2, 0)]));
        manager.add_block(3, Block::new("TNT", true, vec![
            IVec2::new(8, 0),
            IVec2::new(8, 0),
            IVec2::new(9, 0),
            IVec2::new(10, 0),
            IVec2::new(8, 0),
            IVec2::new(8, 0)
        ]));
        manager.add_block(4, Block::new("Cloud", false, vec![IVec2::new(2, 4)]));

        let gizmo = VertexBuffer::from_mesh(Mesh {
            positions: Some(vec![
                // x
                Vec3::new(-0.2, 0.0, 0.0),
                Vec3::new(-0.2, 0.2, 0.2),
                Vec3::new(-0.2, 0.2, 0.2),
                Vec3::new(-0.2, 0.2, 0.2),
                Vec3::new(-0.2, 0.0, 0.0),
                Vec3::new(-0.2, 0.0, 0.0),

                // y
                Vec3::new(0.0, -0.2, 0.0),
                Vec3::new(0.2, -0.2, 0.0),
                Vec3::new(0.2, -0.2, 0.2),
                Vec3::new(0.2, -0.2, 0.2),
                Vec3::new(0.0, -0.2, 0.2),
                Vec3::new(0.0, -0.2, 0.0),

                // z
                Vec3::new(0.0, 0.0, -0.2),
                Vec3::new(0.2, 0.0, -0.2),
                Vec3::new(0.2, 0.2, -0.2),
                Vec3::new(0.2, 0.2, -0.2),
                Vec3::new(0.0, 0.2, -0.2),
                Vec3::new(0.0, 0.0, -0.2),
            ]),

            normals: Some(vec![
                // x
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),

                // y
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),

                // z
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 1.0),
            ]),
            tex_coords: None,
            occlusion: None
        });

        let gizmo_shader = Shader::new(vec![Box::new((gl::FRAGMENT_SHADER, "res/shaders/gizmo.fs.glsl".to_string())),
                                            Box::new((gl::VERTEX_SHADER, "res/shaders/gizmo.vs.glsl".to_string()))]).unwrap();

        let a = TestActor::new(&manager);
        Game { actors: vec![Box::new(a)], g_buffer: g_buffer, quad: quad, def: def.unwrap(),
                camera: RefCell::new(FpsCamera::new(cgmath::Point3::new(0.0, 0.0, 0.0), 0.5)),
                gizmo: gizmo, gizmo_shader: gizmo_shader
        }
    }

    pub fn render(&self) {
        self.g_buffer.bind(); check_gl!();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        for actor in self.actors.iter() {
            actor.render(self.camera.borrow());
        }

        RenderTarget::reset(); check_gl!();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.def.bind(); check_gl!();
        self.def.uniform_texture("_Albedo".to_string(), &self.g_buffer.color_attachments[0], 0); check_gl!();
        self.def.uniform_texture("_Normal".to_string(), &self.g_buffer.color_attachments[1], 1); 
        self.def.uniform_texture("_Position".to_string(), &self.g_buffer.color_attachments[2], 2); 
        self.def.uniform_texture("_Occlusion".to_string(), &self.g_buffer.color_attachments[3], 3);
        self.quad.render();

        //let roty = cgmath::Matrix4::from_angle_y(cgmath::Rad(self.camera.borrow().hor_angle));
        //let rotx = cgmath::Matrix4::from_angle_x(cgmath::Rad(self.camera.borrow().ver_angle));

        //self.gizmo_shader.bind();
        //self.gizmo_shader.uniform_mat4x4("_RotX".to_string(), rotx);
        //self.gizmo_shader.uniform_mat4x4("_RotY".to_string(), roty);
        //self.gizmo.render();

        //unsafe { gl::Disable(gl::DEPTH_TEST); }
        //self.gizmo_shader.bind();
        //self.gizmo.render();
        //unsafe { gl::Enable(gl::DEPTH_TEST); }
    }

    pub fn update(&self) {
        for actor in self.actors.iter() {
            actor.update();
        }
    }
    
    pub fn keyboard_input(&self, input: glutin::KeyboardInput) {
        self.camera.borrow_mut().keyboard_input(input);
    }

    pub fn mouse_input(&self, position: glutin::dpi::LogicalPosition) {
        self.camera.borrow_mut().mouse_input(position);
    }
}
