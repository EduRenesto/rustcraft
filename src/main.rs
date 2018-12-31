extern crate cgmath;
extern crate glutin;
extern crate gl;
extern crate stb_image;
extern crate noise;

use glutin::GlContext;

use std::sync::Arc;

#[macro_use]
pub mod gl_error;

pub mod game;
pub mod mesh;
pub mod vertex_buffer;
pub mod shader;
pub mod texture;
pub mod render_target;
pub mod actor;
pub mod test_actor;
pub mod block;
pub mod chunk;
pub mod world;

#[macro_use]
pub mod block_manager;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Rustcraft")
        .with_dimensions(glutin::dpi::LogicalSize::new(1280.0, 768.0));
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop)
        .expect("Failed to create window!");

    unsafe {
        use glutin::GlContext;
        gl_window.make_current().expect("Failed to make GL context current!");
    }

    gl::load_with(|s| gl_window.get_proc_address(s) as *const _);

    let game = Arc::new(game::Game::new());

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Viewport(0, 0, 1280, 720);
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut run = true;

    while run {
        events_loop.poll_events(|evt| {
            match evt {
                glutin::Event::WindowEvent {
                    event: glutin::WindowEvent::CloseRequested, 
                    ..
                } => {
                    run = false;
                }
                _ => {
                }
            }
        });

        game.update();
        game.render();

        gl_window.swap_buffers().unwrap();
    }
}
