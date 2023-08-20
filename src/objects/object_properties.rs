use crate::hitrecord::HitRecord;
use crate::materials::material_properties::Material;
use crate::ray::Ray;
use glam::Vec3;
use std::rc::Rc;

pub struct ObjectProperties {
    pub position: Vec3,
    pub material: Rc<dyn Material>,
}

impl ObjectProperties {
    pub const fn new(position: Vec3, material: Rc<dyn Material>) -> Self {
        Self { position, material }
    }
}

pub trait Object {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord, is_shadow_ray: bool) -> bool;
    fn update(&self, delta_time: f32);
}
