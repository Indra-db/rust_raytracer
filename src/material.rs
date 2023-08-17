use crate::ray::HitRecord;

use glam::Vec3;

pub type RGBColor = Vec3;

struct MaterialProperties {
    diffuse_color: RGBColor,
    diffuse_reflectance: f32,
    reflectiveness_environment: f32,
}

pub struct LambertianMaterial {
    properties: MaterialProperties,
}

pub struct LambertPhongMaterial {
    properties: MaterialProperties,
    specular_reflectance: f32,
    phong_exponent: i32,
}

pub struct PhongBRDF {
    properties: MaterialProperties,
    albedo: RGBColor,
    roughness: f32,
    is_metal: bool,
}

trait Material {
    fn shade(
        &self,
        hitrecord: &HitRecord,
        light_direction: &Vec3,
        view_direction: &Vec3,
    ) -> RGBColor;
}

// Usage
//fn some_function<T: Material>(mat: &T) {
//    let color = mat.shade(...);
//}
