use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    t_min: f32,
    t_max: f32,
}

impl Default for Ray {
    fn default() -> Self {
        Self { origin: Vec3::default(), direction: Vec3::default(), t_min: 0.0001, t_max: f32::MAX }
    }
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction, t_min: 0.0001, t_max: f32::MAX }
    }

    pub fn new_with_limits(origin: Vec3, direction: Vec3, t_min: f32, t_max: f32) -> Self {
        Self { origin, direction, t_min, t_max }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
