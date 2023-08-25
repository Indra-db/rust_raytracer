use crate::hitrecord::HitRecord;
use crate::materials::material_properties::{Material, MaterialProperties, RGBColor};
use crate::math::brdf;
use glam::Vec3;

pub struct PhongBRDF {
    pub properties: MaterialProperties,
    pub albedo: RGBColor,
    pub roughness: f32,
    pub is_metal: bool,
}

impl PhongBRDF {
    pub const fn new(
        albedo: RGBColor,
        roughness: f32,
        is_metal: bool,
        diffuse_reflectance: f32,
        reflectiveness_environment: f32,
    ) -> Self {
        let properties = if is_metal {
            MaterialProperties::new(RGBColor::ZERO, diffuse_reflectance, reflectiveness_environment)
        } else {
            MaterialProperties::new(albedo, diffuse_reflectance, reflectiveness_environment)
        };

        Self { properties, albedo, roughness, is_metal }
    }
}

impl Material for PhongBRDF {
    #[allow(clippy::cast_precision_loss)]
    fn shade(
        &self,
        hitrecord: &HitRecord<'_>,
        light_direction: &Vec3,
        view_direction: &Vec3,
    ) -> RGBColor {
        let half_vector = (*light_direction + *view_direction).normalize();
        let fresnel: RGBColor = brdf::schlick(&half_vector, view_direction, &self.albedo);
        let d = brdf::trowbridge_reitz_ggx(&hitrecord.normal, &half_vector, self.roughness);
        let k = self.roughness.mul_add(self.roughness, 1.0).powi(2) / 8.0;
        let g = brdf::smith_method(&hitrecord.normal, view_direction, light_direction, k);
        let kd: RGBColor = (RGBColor::ONE - fresnel) * i32::from(!self.is_metal) as f32;

        let specular = fresnel * d * g
            / (4.0
                * hitrecord.normal.dot(*light_direction).max(std::f32::EPSILON)
                * hitrecord.normal.dot(*view_direction).max(std::f32::EPSILON));

        let diffuse = brdf::lambert_color(&self.properties.diffuse_color, &kd);
        specular + diffuse
    }

    fn get_reflectiveness_environment(&self) -> f32 {
        self.properties.reflectiveness_environment
    }
}
