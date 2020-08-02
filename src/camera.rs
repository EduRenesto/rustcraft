pub trait Camera {
    fn get_view_matrix(&self) -> cgmath::Matrix4<f32>;
    fn get_projection_matrix(&self) -> cgmath::Matrix4<f32>;
}
