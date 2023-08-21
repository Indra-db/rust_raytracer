use super::lambert_material::LambertianMaterial;
use super::material_definitions::{
    create_diffuse_rgb_hash_map, create_linear_fresnel_hash_map, RGBColor,
};
use super::material_properties::Material;
use std::collections::HashMap;

enum RoughnessConstants {
    Smooth,
    HalfRough,
    Rough,
}

impl RoughnessConstants {
    fn value(&self) -> f32 {
        match *self {
            RoughnessConstants::Smooth => 0.1,
            RoughnessConstants::HalfRough => 0.6,
            RoughnessConstants::Rough => 1.0,
        }
    }
}

pub struct MaterialManager<'a> {
    linear_freshnel: HashMap<&'a str, RGBColor>,
    diffuse_colors: HashMap<&'a str, RGBColor>,
    pub materials: HashMap<String, Box<dyn Material>>,
}

impl<'a> MaterialManager<'a> {
    const DIFFUSE_REFLECTANCE: f32 = 1.0;
    const SPECULAR_REFLECTANCE: f32 = 1.0;
    const PHONG_EXPONENT: f32 = 60.0;

    pub fn new() -> Self {
        MaterialManager {
            linear_freshnel: create_linear_fresnel_hash_map(),
            diffuse_colors: create_diffuse_rgb_hash_map(),
            materials: HashMap::new(),
        }
    }

    pub fn add_material(&mut self, name: String, material: Box<dyn Material>) {
        if !self.materials.contains_key(&name) {
            self.materials.insert(name, material);
        }
    }

    pub fn get_material(&self, name: &str) -> Option<&dyn Material> {
        self.materials.get(name).map(AsRef::as_ref)
    }

    pub fn add_lambert_material(&mut self, color_name: &str, reflectiveness: i32) {
        let material_id: String = format!("lambert_{}_RE{}", color_name, reflectiveness);

        if self.materials.contains_key(&material_id) {
            return;
        }

        let diffuse_color = match self.diffuse_colors.get(color_name) {
            Some(color) => *color / 255.0,
            None => {
                println!("Color {} not found", color_name);
                return;
            }
        };

        let reflectiveness = Self::map_reflectiveness_input_from_int_to_float(reflectiveness);
        let new_material = Box::new(LambertianMaterial::new(
            diffuse_color,
            Self::DIFFUSE_REFLECTANCE,
            reflectiveness,
        ));

        self.add_material(material_id, new_material);
    }

    fn map_reflectiveness_input_from_int_to_float(reflectiveness: i32) -> f32 {
        match reflectiveness {
            0 => 0.0,
            1 => 0.1,
            2 => 0.2,
            3 => 0.3,
            4 => 0.4,
            5 => 0.5,
            6 => 0.6,
            7 => 0.7,
            8 => 0.8,
            9 => 0.9,
            _ => 1.0,
        }
    }
}
