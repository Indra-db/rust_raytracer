use super::object_properties::{Object, ObjectProperties};
use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use glam::Vec3;

#[derive(Copy, Clone)]
pub enum CullMode {
    None,
    BackFace,
    FrontFace,
}

pub struct Triangle<'mm> {
    pub object_properties: ObjectProperties<'mm>,
    pub vertices: [Vec3; 3],
    pub normal: Vec3,
    pub cull_mode: CullMode,
}

impl<'mm> Triangle<'mm> {
    pub fn new(
        object_properties: ObjectProperties<'mm>,
        vertices: [Vec3; 3],
        cull_mode: CullMode,
    ) -> Self {
        let normal = (vertices[2] - vertices[0]).cross(vertices[1] - vertices[0]).normalize();
        Self { object_properties, vertices, normal, cull_mode }
    }
}

impl<'mm> Object<'mm> for Triangle<'mm> {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord<'mm>, is_shadow_ray: bool) -> bool {
        let v_dot_n = ray.direction.dot(self.normal);

        match self.cull_mode {
            CullMode::None => {}
            CullMode::BackFace => {
                //if positive, then the triangle is facing away from the camera
                if v_dot_n > 0.0 {
                    return false;
                }
            }
            CullMode::FrontFace => {
                if v_dot_n < 0.0 {
                    return false;
                }
            }
        }

        if v_dot_n.abs() < f32::EPSILON {
            return false; // ray is parallel to the triangle
        }

        // Calculate t using a vertex of the triangle instead of the center
        let t = (self.vertices[0] + self.object_properties.position - ray.origin).dot(self.normal)
            / v_dot_n;

        if t < ray.t_min || t > ray.t_max || (t > hit_record.t && !is_shadow_ray) {
            return false;
        }

        let hitpoint = ray.origin + t * ray.direction;

        // Barycentric containment test for the intersection point
        for i in 0..3 {
            let edge = self.vertices[(i + 1) % 3] + self.object_properties.position
                - (self.vertices[i] + self.object_properties.position);
            let side_to_point = self.vertices[i] + self.object_properties.position - hitpoint;
            if self.normal.dot(edge.cross(side_to_point)) < 0.0 {
                return false;
            }
        }

        hit_record.t = t;
        hit_record.hitpoint = hitpoint;
        hit_record.normal = self.normal;
        hit_record.material = Some(self.object_properties.material);

        true
    }

    fn update(&self, _delta_time: f32) {}
}
