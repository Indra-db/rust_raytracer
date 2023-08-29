use glam::{IVec2, Mat4, Vec3, Vec4};
use sdl2::keyboard::Keycode;

pub struct Camera {
    pub look_at: Mat4,
    pub forward_vector: Vec4,
    pub right_vector: Vec4,
    pub up_vector: Vec4,
    pub position: Vec3,
    fov: f32,
    pub speed: f32,
    scale_factor: f32,
    update_look_at: bool,
}

impl Camera {
    const WORLD_UP_VECTOR: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };

    pub fn new(position: Vec3, fov: f32) -> Self {
        let mut camera = Self {
            look_at: Mat4::default(),
            forward_vector: Vec4::new(0.0, 0.0, 1.0, 0.0),
            right_vector: Vec4::default(),
            up_vector: Vec4::default(),
            position,
            fov,
            speed: 15.0,
            scale_factor: (fov.to_radians() / 2.0).tan(),
            update_look_at: true,
        };
        camera.calculate_look_at();
        camera
    }

    #[inline]
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.scale_factor = (fov.to_radians() / 2.0).tan();
    }

    #[inline]
    pub fn get_forward_vector(&self) -> Vec3 {
        self.forward_vector.truncate()
    }

    #[inline]
    pub fn get_fov_radians(&self) -> f32 {
        self.fov.to_radians()
    }

    #[inline]
    pub const fn get_fov_degrees(&self) -> f32 {
        self.fov
    }

    #[inline]
    pub const fn get_scale_factor(&self) -> f32 {
        self.scale_factor
    }

    pub fn update_look_at(&mut self) {
        if self.update_look_at {
            self.calculate_look_at();
        }
    }

    pub fn camera_translation(&mut self, delta_time: f32, sdl_keycode: Keycode) {
        match sdl_keycode {
            Keycode::W => {
                self.position -= self.forward_vector.truncate() * self.speed * delta_time;
            }
            Keycode::S => {
                self.position += self.forward_vector.truncate() * self.speed * delta_time;
            }
            Keycode::A => {
                self.position -= self.right_vector.truncate() * self.speed * delta_time;
            }
            Keycode::D => {
                self.position += self.right_vector.truncate() * self.speed * delta_time;
            }
            Keycode::Q => {
                self.position += Self::WORLD_UP_VECTOR * self.speed * delta_time;
            }
            Keycode::E => {
                self.position -= Self::WORLD_UP_VECTOR * self.speed * delta_time;
            }
            _ => {}
        }

        self.update_look_at = true;
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn camera_rotation(&mut self, _delta_time: f32, mouse_position: IVec2) {
        const ROTATION_SENSIVITY: f32 = 0.035;

        self.pitch(-mouse_position.y as f32 * ROTATION_SENSIVITY);
        self.yaw(-mouse_position.x as f32 * ROTATION_SENSIVITY);
        self.update_look_at = true;
    }

    fn pitch(&mut self, angle: f32) {
        let rotation = Mat4::from_axis_angle(self.right_vector.truncate(), angle.to_radians());
        self.forward_vector = rotation.transpose().inverse() * self.forward_vector;
        self.forward_vector = rotation * self.forward_vector;
        self.update_look_at = true;
    }

    fn yaw(&mut self, angle: f32) {
        let rotation = Mat4::from_axis_angle(self.up_vector.truncate(), angle.to_radians());
        self.forward_vector = rotation.transpose().inverse() * self.forward_vector;
        self.forward_vector = rotation * self.forward_vector;
        self.update_look_at = true;
    }

    fn calculate_look_at(&mut self) {
        self.right_vector =
            Self::WORLD_UP_VECTOR.cross(self.forward_vector.truncate()).normalize().extend(0.0);

        self.up_vector = self
            .forward_vector
            .truncate()
            .cross(self.right_vector.truncate())
            .normalize()
            .extend(0.0);

        self.look_at = Mat4 {
            x_axis: self.right_vector.truncate().extend(0.0),
            y_axis: self.up_vector.truncate().extend(0.0),
            z_axis: self.forward_vector.truncate().extend(0.0),
            w_axis: Vec4::new(self.position.x, self.position.y, self.position.z, 1.0),
        }
    }
}
