use crate::camera::Camera;

pub trait Actor {
    fn update(&mut self);
    fn render(&mut self, camera: &dyn Camera);
}
