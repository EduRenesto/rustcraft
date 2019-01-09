use std::cell::Ref;
use crate::fps_camera::FpsCamera;

pub trait Actor {
    fn update(&self);
    fn render(&self, camera: Ref<FpsCamera>);
}
