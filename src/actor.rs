use crate::camera::Camera;

pub trait Actor {
    fn update(&self);
    fn render(&self, camera: &dyn Camera);
}
