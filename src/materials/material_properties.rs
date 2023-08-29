use crate::hitrecord::HitRecord;
use glam::Vec3;
pub type RGBColor = Vec3;

pub struct MaterialProperties {
    pub diffuse_color: RGBColor,
    pub diffuse_reflectance: f32,
    pub reflectiveness_environment: f32,
}

impl MaterialProperties {
    pub const fn new(diffuse_color: RGBColor, diffuse_reflectance: f32, reflectiveness_environment: f32) -> Self {
        Self { diffuse_color, diffuse_reflectance, reflectiveness_environment }
    }
}

pub trait Material: Sync + Send {
    fn shade(&self, hitrecord: &HitRecord<'_>, light_direction: &Vec3, view_direction: &Vec3) -> RGBColor;

    fn get_reflectiveness_environment(&self) -> f32;
}

pub struct DefaultMaterial {}

impl Material for DefaultMaterial {
    fn shade(&self, _hitrecord: &HitRecord<'_>, _light_direction: &Vec3, _view_direction: &Vec3) -> RGBColor {
        RGBColor::ZERO
    }

    fn get_reflectiveness_environment(&self) -> f32 {
        0.0
    }
}
