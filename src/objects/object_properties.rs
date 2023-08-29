use crate::hitrecord::HitRecord;
use crate::materials::MaterialEnum;
use crate::ray::Ray;
use enum_dispatch::enum_dispatch;
use glam::Vec3;

#[derive(Clone)]
pub struct ObjectProperties<'mm> {
    pub position: Vec3,
    pub material: &'mm MaterialEnum,
}

impl<'mm> ObjectProperties<'mm> {
    pub const fn new(position: Vec3, material: &'mm MaterialEnum) -> Self {
        Self { position, material }
    }
}

#[enum_dispatch]
pub trait Object<'mm>: Sync + Send {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord<'mm>, is_shadow_ray: bool) -> bool;
    fn update(&self, delta_time: f32);
}
