use enum_dispatch::enum_dispatch;

use self::{
    object_properties::Object, plane::Plane, sphere::Sphere, triangle::Triangle,
    triangle_mesh::TriangleMesh,
};
use crate::{hitrecord::HitRecord, ray::Ray};

pub mod object_properties;
pub mod plane;
pub mod sphere;
pub mod triangle;
pub mod triangle_mesh;

#[enum_dispatch(Object)]
pub enum ObjectEnum<'mm> {
    Plane(Plane<'mm>),
    Sphere(Sphere<'mm>),
    Triangle(Triangle<'mm>),
    TriangleMesh(TriangleMesh<'mm>),
}
