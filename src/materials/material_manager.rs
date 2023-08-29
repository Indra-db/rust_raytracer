use super::lambert_material::LambertMaterial;
use super::lambert_phong_material::LambertPhongMaterial;
use super::material_definitions::{
    create_diffuse_rgb_hash_map, create_linear_fresnel_hash_map, RGBColor,
};
use super::material_properties::{Material, MaterialProperties};
use super::phong_brdf_material::PhongBRDFMaterial;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum RoughnessConstants {
    Smooth,
    HalfRough,
    Rough,
}

impl RoughnessConstants {
    const fn value(self) -> f32 {
        match self {
            Self::Smooth => 0.1,
            Self::HalfRough => 0.6,
            Self::Rough => 1.0,
        }
    }
    const fn string(&self) -> &str {
        match *self {
            Self::Smooth => "Smooth",
            Self::HalfRough => "HalfRough",
            Self::Rough => "Rough",
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
        MaterialManager {
            linear_freshnel: create_linear_fresnel_hash_map(),
            diffuse_colors: create_diffuse_rgb_hash_map(),
            materials: HashMap::new(),
        }
    }

    pub fn add_material(&mut self, name: String, material: Box<dyn Material>) {
        self.materials.entry(name).or_insert(material);
    }

    pub fn get_material(&self, name: &str) -> Option<&dyn Material> {
        self.materials.get(name).map(AsRef::as_ref)
    }

    pub fn add_lambert_material(&mut self, color_name: &str, reflectiveness: i32) {
        let material_id: String = format!("lambert_{color_name}_RE{reflectiveness}");

        if self.materials.contains_key(&material_id) {
            return;
        }

        let diffuse_color = if let Some(color) = self.diffuse_colors.get(color_name) {
            *color / 255.0
        } else {
            println!("Color {color_name} not found");
            return;
        };

        let reflectiveness = Self::map_reflectiveness_input_from_int_to_float(reflectiveness);
        let new_material = Box::new(LambertMaterial::new(
            diffuse_color,
            Self::DIFFUSE_REFLECTANCE,
            reflectiveness,
        ));

        self.add_material(material_id, new_material);
    }

    pub fn add_lambert_phong_material(&mut self, color_name: &str, reflectiveness: i32) {
        let material_id: String = format!("lambert_phong_{color_name}_RE{reflectiveness}");

        if self.materials.contains_key(&material_id) {
            return;
        }

        let diffuse_color = if let Some(color) = self.diffuse_colors.get(color_name) {
            *color / 255.0
        } else {
            println!("Color {color_name} not found");
            return;
        };

        let reflectiveness = Self::map_reflectiveness_input_from_int_to_float(reflectiveness);
        let new_material = Box::new(LambertPhongMaterial::new(
            MaterialProperties::new(diffuse_color, Self::DIFFUSE_REFLECTANCE, reflectiveness),
            Self::SPECULAR_REFLECTANCE,
            Self::PHONG_EXPONENT,
        ));

        self.add_material(material_id, new_material);
    }

    pub fn add_phong_brdf_metal_material(&mut self, albedo: &str, roughness: RoughnessConstants) {
        let material_id: String = format!("phong_brdf_{}_metal_{}", albedo, roughness.string());

        if self.materials.contains_key(&material_id) {
            return;
        }

        let fresnel_value = if let Some(color) = self.linear_freshnel.get(albedo) {
            *color
        } else {
            println!("Color {albedo} not found");
            return;
        };

        let roughness_value = roughness.value();
        let new_material = Box::new(PhongBRDFMaterial::new(
            fresnel_value,
            roughness_value,
            true,
            Self::DIFFUSE_REFLECTANCE,
            1.0,
        ));

        self.add_material(material_id, new_material);
    }

    pub fn add_phong_brdf_dielectric_material(
        &mut self,
        albedo: &str,
        roughness: RoughnessConstants,
        reflectiveness: i32,
    ) {
        let material_id: String =
            format!("phong_brdf_{}_dielectric_{}_RE{}", albedo, roughness.string(), reflectiveness);

        if self.materials.contains_key(&material_id) {
            return;
        }

        let fresnel_value = if let Some(color) = self.diffuse_colors.get(albedo) {
            *color / 255.0
        } else {
            println!("Color {albedo} not found");
            return;
        };

        let roughness_value = roughness.value();
        let reflectiveness_value = Self::map_reflectiveness_input_from_int_to_float(reflectiveness);
        let new_material = Box::new(PhongBRDFMaterial::new(
            fresnel_value,
            roughness_value,
            false,
            Self::DIFFUSE_REFLECTANCE,
            reflectiveness_value,
        ));

        self.add_material(material_id, new_material);
    }

    const fn map_reflectiveness_input_from_int_to_float(reflectiveness: i32) -> f32 {
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
