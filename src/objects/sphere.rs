use super::object_properties::{Object, ObjectProperties};
use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use glam::Vec3;

pub struct Sphere {
    pub object_properties: ObjectProperties,
    pub radius: f32,
}

impl Sphere {
    pub fn new(object_properties: ObjectProperties, radius: f32) -> Self {
        Self { object_properties, radius }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord, _is_shadow_ray: bool) -> bool {
        // Vector from the ray origin to the sphere center
        let ray_to_sphere = self.object_properties.position - ray.origin;

        // Square of the distance from the sphere center to the ray's closest approach
        let approach_distance_sq = ray_to_sphere.reject_from(ray.direction).length_squared();
        let radius_sq = self.radius.powi(2);

        // If ray's closest approach is outside of the sphere or tangential to the sphere
        if approach_distance_sq > radius_sq
            || (approach_distance_sq - radius_sq).abs() < f32::EPSILON
        {
            return false;
        }

        let tca = ray_to_sphere.dot(ray.direction);
        let thc = (radius_sq - approach_distance_sq).sqrt();

        // Distance from ray origin to the first intersection point
        let mut t0 = tca - thc;

        // Check if t0 is within the ray's bounds
        if t0 < ray.t_min || t0 > ray.t_max {
            t0 = tca + thc;
        }

        if t0 < ray.t_min || t0 > ray.t_max {
            return false;
        }

        hit_record.t = t0;
        hit_record.hitpoint = ray.origin + t0 * ray.direction;
        hit_record.normal = (hit_record.hitpoint - self.object_properties.position).normalize();
        hit_record.material = self.object_properties.material.clone();

        true
    }

    fn update(&self, _delta_time: f32) {}
}
