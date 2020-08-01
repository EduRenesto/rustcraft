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
pub mod fps_camera;

#[macro_use]
pub mod block_manager;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Rustcraft")
        .with_dimensions(glutin::dpi::LogicalSize::new(1280.0, 720.0));
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop)
        .expect("Failed to create window!");

    unsafe {
        gl_window.make_current().expect("Failed to make GL context current!");
    }

    gl::load_with(|s| gl_window.get_proc_address(s) as *const _);

    let game = Arc::new(game::Game::new());

    unsafe {
        gl::Viewport(0, 0, 1280, 720);
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut run = true;
    let mut paused = false;

    while run {
        events_loop.poll_events(|evt| {
            match evt {
                //glutin::Event::WindowEvent {
                    //event: glutin::WindowEvent::CloseRequested, 
                    //..
                //} => {
                    //run = false;
                //}, 
                //glutin::Event::WindowEvent {
                    //event: glutin::WindowEvent::KeyboardInput { input: glutin::KeyboardInput {..}, .. },
                    //..
                //} => {
                    //game.keyboard_input(input)
                //}
                //_ => {
                //}
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::CloseRequested => run = false,
                        glutin::WindowEvent::CursorMoved { position, .. } => {
                            if !paused {
                                game.mouse_input(position);
                            }
                        },
                        _ => {}
                    }
                }, 
                glutin::Event::DeviceEvent { event, .. } => {
                    match event {
                        glutin::DeviceEvent::Key(input) => {
                            if let Some(k) = input.virtual_keycode {
                                if k == glutin::VirtualKeyCode::Escape && input.state == glutin::ElementState::Released {
                                    paused = !paused;
                                }
                            }

                            if !paused {
                                game.keyboard_input(input);
                            }    
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        });

        game.update();
        game.render();

        gl_window.swap_buffers().unwrap();

        if !paused {
            gl_window.set_cursor_position(
                glutin::dpi::LogicalPosition::new(1280.0/2.0, 720.0/2.0)).unwrap();
        }
    }
}
