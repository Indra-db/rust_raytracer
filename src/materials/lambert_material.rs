use crate::hitrecord::HitRecord;
use crate::materials::material_properties::{Material, MaterialProperties};
use crate::math::brdf;
use glam::Vec3;
pub type RGBColor = Vec3;

pub struct LambertianMaterial {
    pub properties: MaterialProperties,
}

impl LambertianMaterial {
    pub const fn new(
        diffuse_color: RGBColor,
        diffuse_reflectance: f32,
        reflectiveness_environment: f32,
    ) -> Self {
        Self {
            properties: MaterialProperties {
                diffuse_color,
                diffuse_reflectance,
                reflectiveness_environment,
            },
        }
    }
}
impl Material for LambertianMaterial {
    fn shade(
        &self,
        _hitrecord: &HitRecord<'_>,
        _light_direction: &Vec3,
        _view_direction: &Vec3,
    ) -> RGBColor {
        brdf::lambert(&self.properties.diffuse_color, self.properties.diffuse_reflectance)
    }

    fn get_reflectiveness_environment(&self) -> f32 {
        self.properties.reflectiveness_environment
    }
}
