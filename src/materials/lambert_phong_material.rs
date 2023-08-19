use crate::materials::material_properties::*;

pub struct LambertPhongMaterial {
    properties: MaterialProperties,
    specular_reflectance: f32,
    phong_exponent: i32,
}
