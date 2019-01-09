pub struct FpsCamera {
    position: cgmath::Point3<f32>,
    speed: f32,

    hor_angle: f32,
    ver_angle: f32,
    direction: cgmath::Vector3<f32>,
    right: cgmath::Vector3<f32>
}

impl FpsCamera {
    pub fn new(pos: cgmath::Point3<f32>, speed: f32) -> FpsCamera {
        FpsCamera { 
            position: pos, 
            hor_angle: 0.0, 
            ver_angle: 0.0, 
            speed: speed,
            direction: cgmath::Vector3::new(0.0, 0.0, 1.0),
            right: cgmath::Vector3::new(1.0, 0.0, 0.0)
        }
    }

    pub fn keyboard_input(&mut self, input: glutin::KeyboardInput) {
        match input.scancode {
            17 => {
                self.position += self.direction * self.speed;
            },
            30 => {
                self.position += self.right * -self.speed;
            },
            31 => {
                self.position += self.direction * -self.speed;
            },
            32 => {
                self.position += self.right * self.speed;
            },
            _ => {}
        }
    }
    
    pub fn mouse_input(&mut self, pos: glutin::dpi::LogicalPosition) {
        self.hor_angle += 0.001 * (1280.0/2.0 - pos.x as f32);
        self.ver_angle += 0.001 * (720.0/2.0 - pos.y as f32);

        self.direction = cgmath::Vector3::new(self.ver_angle.cos() * self.hor_angle.sin(), 
                                             self.ver_angle.sin(),
                                             self.ver_angle.cos() * self.hor_angle.cos());

        let half_pi = std::f32::consts::FRAC_PI_2;
        self.right = cgmath::Vector3::new((self.hor_angle - half_pi).sin(), 0.0, (self.hor_angle - half_pi).cos());
    }

    pub fn get_view_matrix(&self) -> cgmath::Matrix4<f32> {

        let up = self.right.cross(self.direction);

        cgmath::Matrix4::look_at(self.position, self.position + self.direction, up)
    }
}
