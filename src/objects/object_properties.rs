use crate::hitrecord::HitRecord;
use crate::materials::material_properties::Material;
use crate::ray::Ray;
use glam::Vec3;
use std::rc::Rc;

pub struct ObjectProperties<'mm> {
    pub position: Vec3,
    pub material: &'mm dyn Material<'mm>,
}

impl<'mm> ObjectProperties<'mm> {
    pub const fn new(position: Vec3, material: &'mm dyn Material<'mm>) -> Self {
        Self { position, material }
    }
}

pub trait Object<'mm> {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord<'mm>, is_shadow_ray: bool) -> bool;
    fn update(&self, delta_time: f32);
}