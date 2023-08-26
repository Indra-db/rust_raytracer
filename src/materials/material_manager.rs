use super::lambert_material::LambertianMaterial;
use super::lambert_phong_material::LambertPhongMaterial;
use super::material_definitions::{create_diffuse_rgb_hash_map, create_linear_fresnel_hash_map, RGBColor};
use super::material_properties::{Material, MaterialProperties};
use super::phong_brdf_material::PhongBRDF;
use std::collections::HashMap;

pub enum RoughnessConstants {
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
    fn string(&self) -> &str {
        match *self {
            RoughnessConstants::Smooth => "Smooth",
            RoughnessConstants::HalfRough => "HalfRough",
            RoughnessConstants::Rough => "Rough",
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
    const PHONG_EXPONENT: i32 = 60;

    pub fn new() -> Self {
        MaterialManager { linear_freshnel: create_linear_fresnel_hash_map(), diffuse_colors: create_diffuse_rgb_hash_map(), materials: HashMap::new() }
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
        let new_material = Box::new(LambertianMaterial::new(diffuse_color, Self::DIFFUSE_REFLECTANCE, reflectiveness));

        self.add_material(material_id, new_material);
    }

    pub fn add_lambert_phong_material(&mut self, color_name: &str, reflectiveness: i32) {
        let material_id: String = format!("lambert_phong_{}_RE{}", color_name, reflectiveness);

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
        let new_material = Box::new(LambertPhongMaterial::new(MaterialProperties::new(diffuse_color, Self::DIFFUSE_REFLECTANCE, reflectiveness), Self::SPECULAR_REFLECTANCE, Self::PHONG_EXPONENT));

        self.add_material(material_id, new_material);
    }

    pub fn add_phong_brdf_metal_material(&mut self, albedo: &str, roughness: RoughnessConstants) {
        let material_id: String = format!("phong_brdf_{}_metal_{}", albedo, roughness.string());

        if self.materials.contains_key(&material_id) {
            return;
        }

        let fresnel_value = match self.linear_freshnel.get(albedo) {
            Some(color) => *color,
            None => {
                println!("Color {} not found", albedo);
                return;
            }
        };

        let roughness_value = roughness.value();
        let new_material = Box::new(PhongBRDF::new(fresnel_value, roughness_value, true, Self::DIFFUSE_REFLECTANCE, 1.0));

        self.add_material(material_id, new_material);
    }

    pub fn add_phong_brdf_dielectric_material(&mut self, albedo: &str, roughness: RoughnessConstants, reflectiveness: i32) {
        let material_id: String = format!("phong_brdf_{}_dielectric_{}_RE{}", albedo, roughness.string(), reflectiveness);

        if self.materials.contains_key(&material_id) {
            return;
        }

        let fresnel_value = match self.diffuse_colors.get(albedo) {
            Some(color) => *color / 255.0,
            None => {
                println!("Color {} not found", albedo);
                return;
            }
        };

        let roughness_value = roughness.value();
        let reflectiveness_value = Self::map_reflectiveness_input_from_int_to_float(reflectiveness);
        let new_material = Box::new(PhongBRDF::new(fresnel_value, roughness_value, false, Self::DIFFUSE_REFLECTANCE, reflectiveness_value));

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
