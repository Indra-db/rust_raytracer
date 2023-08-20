use super::object_properties::{Object, ObjectProperties};
use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use glam::Vec3;

pub struct Plane {
    pub object_properties: ObjectProperties,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(object_properties: ObjectProperties, normal: Vec3) -> Self {
        Self { object_properties, normal: normal.normalize() }
    }
}

impl Object for Plane {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord, _is_shadow_ray: bool) -> bool {
        // Calculate the dot product between the ray direction and the plane's normal.
        let ray_dot_normal = ray.direction.dot(self.normal);

        // Can't divide by 0 --> if 0 that means the ray is parallel to the plane
        if ray_dot_normal == 0.0 {
            return false;
        }

        let intersection_distance =
            (self.object_properties.position - ray.origin).dot(self.normal) / ray_dot_normal;

        // If between the interval
        if intersection_distance < ray.t_min || intersection_distance > ray.t_max {
            return false;
        }

        if hit_record.t < intersection_distance {
            return true;
        }

        // Update the hit record
        hit_record.t = intersection_distance;
        hit_record.hitpoint = ray.origin + (intersection_distance * ray.direction);
        hit_record.normal = self.normal;
        hit_record.material = self.object_properties.material.clone();
        true
    }

    fn update(&self, _delta_time: f32) {}
}
