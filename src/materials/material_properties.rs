use crate::hitrecord::HitRecord;
use glam::Vec3;
pub type RGBColor = Vec3;

pub struct MaterialProperties {
    diffuse_color: RGBColor,
    diffuse_reflectance: f32,
    reflectiveness_environment: f32,
}

pub trait Material {
    fn shade(
        &self,
        hitrecord: &HitRecord,
        light_direction: &Vec3,
        view_direction: &Vec3,
    ) -> RGBColor;
}
