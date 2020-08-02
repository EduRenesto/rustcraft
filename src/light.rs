use crate::render_target::RenderTarget;
use crate::camera::Camera;

#[allow(dead_code)]
pub struct Light {
    position: cgmath::Point3<f32>,
    color: cgmath::Vector3<f32>,

    proj_matrix: cgmath::Matrix4<f32>,
    view_matrix: cgmath::Matrix4<f32>,

    pub target: RenderTarget,
}

impl Light {
    pub fn new(position: cgmath::Point3<f32>, color: cgmath::Vector3<f32>) -> Light {
        let proj_matrix = cgmath::ortho(-10.0, 10.0, -10.0, 10.0, 0.1, 10.0);
        let view_matrix = cgmath::Matrix4::look_at(position, cgmath::Point3::new(0.0, 0.0, 0.0), cgmath::Vector3::new(0.0, 1.0, 0.0));

        let target = RenderTarget::new(1280, 720, 0, true);

        Light {
            position,
            color,
            proj_matrix,
            view_matrix,
            target,
        }
    }
}

impl Camera for Light {
    fn get_view_matrix(&self) -> cgmath::Matrix4<f32> {
        self.view_matrix
    }

    fn get_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        self.proj_matrix
    }
}
