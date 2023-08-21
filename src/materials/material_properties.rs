use crate::hitrecord::HitRecord;
use glam::Vec3;
pub type RGBColor = Vec3;

pub struct MaterialProperties {
    pub diffuse_color: RGBColor,
    pub diffuse_reflectance: f32,
    pub reflectiveness_environment: f32,
}

pub trait Material<'mm> {
    fn shade(
        &self,
        hitrecord: &HitRecord<'mm>,
        light_direction: &Vec3,
        view_direction: &Vec3,
    ) -> RGBColor;

    fn get_reflectiveness_environment(&self) -> f32;
}

pub struct DefaultMaterial {}

impl<'mm> Material<'mm> for DefaultMaterial {
    fn shade(
        &self,
        _hitrecord: &HitRecord<'mm>,
        _light_direction: &Vec3,
        _view_direction: &Vec3,
    ) -> RGBColor {
        RGBColor::ZERO
    }

    fn get_reflectiveness_environment(&self) -> f32 {
        0.0
    }
}
