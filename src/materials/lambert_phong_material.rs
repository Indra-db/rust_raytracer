use crate::hitrecord::HitRecord;
use crate::materials::material_properties::{Material, MaterialProperties, RGBColor};
use crate::math::brdf;
use glam::Vec3;

pub struct LambertPhongMaterial {
    pub properties: MaterialProperties,
    pub specular_reflectance: f32,
    pub phong_exponent: i32,
}

impl<'mm> Material for LambertPhongMaterial {
    fn shade(
        &self,
        hitrecord: &HitRecord<'_>,
        light_direction: &Vec3,
        view_direction: &Vec3,
    ) -> RGBColor {
        brdf::lambert(&self.properties.diffuse_color, self.properties.diffuse_reflectance)
            + brdf::phong(
                self.specular_reflectance,
                self.phong_exponent,
                light_direction,
                view_direction,
                &hitrecord.normal,
            )
    }

    fn get_reflectiveness_environment(&self) -> f32 {
        self.properties.reflectiveness_environment
    }
}
