use std::cell::Cell;

pub struct FpsCamera {
    position: Cell<cgmath::Point3<f32>>,
    speed: f32,

    pub hor_angle: f32,
    pub ver_angle: f32,
    pub direction: cgmath::Vector3<f32>,
    right: cgmath::Vector3<f32>,

    // TODO remove this once input is properly done
    movement: cgmath::Vector3<f32>
}

impl FpsCamera {
    pub fn new(pos: cgmath::Point3<f32>, speed: f32) -> FpsCamera {
        FpsCamera { 
            position: Cell::new(pos), 
            hor_angle: 0.0, 
            ver_angle: 0.0, 
            speed: speed,
            direction: cgmath::Vector3::new(0.0, 0.0, 1.0),
            right: cgmath::Vector3::new(1.0, 0.0, 0.0),
            movement: cgmath::Vector3::new(0.0, 0.0, 0.0)
        }
    }

    // TODO offload the movement calculation to another method
    // so we can update the movement vector while we move the mouse
    pub fn keyboard_input(&mut self, input: glutin::KeyboardInput) {
        let mult = match input.state {
            glutin::ElementState::Pressed => 1.0,
            glutin::ElementState::Released => 0.0
        };

        self.movement = mult * match input.scancode {
            17 => {
                self.direction * self.speed
            },
            30 => {
                self.right * -self.speed
            },
            31 => {
                self.direction * -self.speed
            },
            32 => {
                self.right * self.speed
            },
            _ => self.movement
        };
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

        self.position.set(self.position.get() + self.movement);

        cgmath::Matrix4::look_at(self.position.get(), self.position.get() + self.direction, up)
    }
}
