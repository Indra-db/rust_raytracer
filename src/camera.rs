use glam::{IVec2, Mat4, Vec3, Vec3A, Vec4};

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

trait Update {
    fn update(&mut self);
}

impl Camera {
    const WORLD_UP_VECTOR: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };

    pub fn new(position: Vec3, fov: f32, speed: f32, scale_factor: f32) -> Self {
        let mut camera = Self {
            look_at: Mat4::default(),
            forward_vector: Vec4::default(),
            right_vector: Vec4::default(),
            up_vector: Vec4::default(),
            position,
            fov,
            speed,
            scale_factor,
            update_look_at: true,
        };
        camera.calculate_look_at();
        camera
    }

    #[inline]
    pub fn get_fov(&self) -> f32 {
        self.fov
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
    pub fn get_fov_degrees(&self) -> f32 {
        self.fov
    }

    #[inline]
    pub fn get_scale_factor(&self) -> f32 {
        let vec: Vec<&dyn Update> = vec![];
        self.scale_factor
    }

    pub fn update(&mut self, delta_time: f32) {
        self.camera_translation(delta_time);
        self.camera_rotation(delta_time);
        if self.update_look_at {
            self.calculate_look_at();
        }
    }

    fn camera_translation(&mut self, delta_time: f32) {
        let mut mouse_position: IVec2 = IVec2::default();
        unsafe {
            sdl2::sys::SDL_GetRelativeMouseState(
                &mut mouse_position.x as *mut i32,
                &mut mouse_position.y as *mut i32,
            );
        }
    }

    fn camera_rotation(&mut self, delta_time: f32) {
        todo!("Implement camera rotation");
    }

    fn calculate_look_at(&mut self) {
        self.right_vector =
            self.forward_vector.truncate().cross(Self::WORLD_UP_VECTOR).normalize().extend(0.0);

        self.up_vector = self
            .right_vector
            .truncate()
            .cross(self.forward_vector.truncate())
            .normalize()
            .extend(0.0);

        self.look_at = Mat4 {
            x_axis: self.right_vector,
            y_axis: self.up_vector,
            z_axis: self.forward_vector,
            w_axis: Vec4::new(self.position.x, self.position.y, self.position.z, 1.0),
        }
    }
}
