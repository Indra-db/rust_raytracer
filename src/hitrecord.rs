use crate::materials::material_properties::Material;
use glam::Vec3;
use std::rc::Rc;

pub struct HitRecord {
    pub hitpoint: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f32,
}
