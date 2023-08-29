use crate::materials::MaterialEnum;
use glam::Vec3;

#[derive(Clone)]
pub struct HitRecord<'mm> {
    pub hitpoint: Vec3,
    pub normal: Vec3,
    pub material: Option<&'mm MaterialEnum>,
    pub t: f32,
}

impl<'mm> HitRecord<'mm> {
    pub fn new(hitpoint: Vec3, normal: Vec3, material: &'mm MaterialEnum, t: f32) -> Self {
        Self { hitpoint, normal, material: Some(material), t }
    }
}

impl<'mm> Default for HitRecord<'mm> {
    fn default() -> Self {
        Self { hitpoint: Vec3::ZERO, normal: Vec3::ZERO, material: None, t: 0.0 }
    }
}
