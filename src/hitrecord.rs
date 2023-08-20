use crate::materials::material_properties::{DefaultMaterial, Material};
use glam::Vec3;
use std::rc::Rc;

pub struct HitRecord {
    pub hitpoint: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f32,
}

impl HitRecord {
    pub fn new(self, hitpoint: Vec3, normal: Vec3, material: Rc<dyn Material>, t: f32) -> Self {
        Self { hitpoint, normal, material, t }
    }

    pub fn set(&mut self, hitpoint: Vec3, normal: Vec3, material: Rc<dyn Material>, t: f32) {
        self.hitpoint = hitpoint;
        self.normal = normal;
        self.material = material;
        self.t = t;
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            hitpoint: Vec3::ZERO,
            normal: Vec3::ZERO,
            material: Rc::new(DefaultMaterial {}),
            t: 0.0,
        }
    }
}
