use crate::materials::material_properties::{DefaultMaterial, Material};
use glam::Vec3;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord<'mm> {
    pub hitpoint: Vec3,
    pub normal: Vec3,
    pub material: Option<&'mm dyn Material<'mm>>,
    pub t: f32,
}

impl<'mm> HitRecord<'mm> {
    pub fn new(hitpoint: Vec3, normal: Vec3, material: &'mm dyn Material<'mm>, t: f32) -> Self {
        Self { hitpoint, normal, material: Some(material), t }
    }

    pub fn set(&mut self, hitpoint: Vec3, normal: Vec3, material: &'mm dyn Material<'mm>, t: f32) {
        self.hitpoint = hitpoint;
        self.normal = normal;
        self.material = Some(material);
        self.t = t;
    }
}

impl<'mm> Default for HitRecord<'mm> {
    fn default() -> Self {
        Self { hitpoint: Vec3::ZERO, normal: Vec3::ZERO, material: None, t: 0.0 }
    }
}
