use crate::materials::material_properties::*;

pub struct PhongBRDF {
    properties: MaterialProperties,
    albedo: RGBColor,
    roughness: f32,
    is_metal: bool,
}
