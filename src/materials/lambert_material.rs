use crate::hitrecord::HitRecord;
use crate::materials::material_properties::{Material, MaterialProperties};
use crate::math::brdf;
use glam::Vec3;
pub type RGBColor = Vec3;

pub struct LambertianMaterial {
    pub properties: MaterialProperties,
}

impl Material for LambertianMaterial {
    fn shade(
        &self,
        _hitrecord: &HitRecord,
        _light_direction: &Vec3,
        _view_direction: &Vec3,
    ) -> RGBColor {
        brdf::lambert(&self.properties.diffuse_color, self.properties.diffuse_reflectance)
    }
}
