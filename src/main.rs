extern crate cgmath;
extern crate glutin;

mod mesh;

fn main() {
    println!("Starting Rustcraft...");

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
                _ => {}
            }
        });
    }
}
