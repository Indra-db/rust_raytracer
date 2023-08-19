use crate::hitrecord::HitRecord;
use glam::Vec3;
pub type RGBColor = Vec3;

pub struct MaterialProperties {
    pub diffuse_color: RGBColor,
    pub diffuse_reflectance: f32,
    pub reflectiveness_environment: f32,
}

pub trait Material {
    fn shade(
        &self,
        hitrecord: &HitRecord,
        light_direction: &Vec3,
        view_direction: &Vec3,
    ) -> RGBColor;
}
