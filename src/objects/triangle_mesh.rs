use super::{
    object_properties::{Object, ObjectProperties},
    triangle::{CullMode, Triangle},
};
use crate::{hitrecord::HitRecord, ray::Ray};
use glam::Vec3;



use std::fs::File;
use std::io::{self, BufRead};

pub struct TriangleMesh<'mm> {
    pub object_properties: ObjectProperties<'mm>,
    pub triangle_mesh: Vec<Triangle<'mm>>,
}

impl<'mm> TriangleMesh<'mm> {
    pub fn new(
        object_properties: ObjectProperties<'mm>,
        vertex_buffer: Vec<Vec3>,
        index_buffer: Vec<u32>,
        cull_mode: CullMode,
    ) -> Self {
        let mut triangle_mesh = Vec::new();

        for i in 0..index_buffer.len() / 3 {
            let triangle = Triangle::new(
                ObjectProperties { position: object_properties.position, material: object_properties.material },
                [
                    vertex_buffer[index_buffer[i * 3] as usize],
                    vertex_buffer[index_buffer[i * 3 + 1] as usize],
                    vertex_buffer[index_buffer[i * 3 + 2] as usize],
                ],
                cull_mode,
            );
            triangle_mesh.push(triangle);
        }

        Self { object_properties, triangle_mesh }
    }

    pub fn new_from_obj(object_properties: ObjectProperties<'mm>, obj_file_name: &str, cull_mode: CullMode) -> Self {
        let path_to_obj = format!("assets/obj_models/{obj_file_name}.obj");

        let regex_vertices = regex::Regex::new(r"(^v\s+(-?\d+.?\d+)\s(-?\d+.?\d+)\s(-?\d+.?\d+)\s*)").unwrap(); //x
        let regex_faces = regex::Regex::new(r"(^f\s(?:([0-9]+)\s([0-9]+)\s([0-9]+))\s*)").unwrap();
        //x

        let mut vertex_buffer: Vec<Vec3> = Vec::new();
        let mut index_buffer: Vec<u32> = Vec::new();

        let file = File::open(path_to_obj).unwrap();
        let lines = io::BufReader::new(file).lines();

        for line in lines {
            if let Ok(line) = line {
                if let Some(captures) = regex_vertices.captures(&line) {
                    if captures.len() > 3 {
                        let x = captures[2].parse::<f32>().unwrap();
                        let y = captures[3].parse::<f32>().unwrap();
                        let z = captures[4].parse::<f32>().unwrap();
                        vertex_buffer.push(Vec3::new(x, y, z));
                    }
                } else if let Some(captures) = regex_faces.captures(&line) {
                    for i in (2..=4).rev() {
                        let index = captures[i].parse::<u32>().unwrap() - 1;
                        index_buffer.push(index);
                    }
                }
            }
        }
        let mut triangle_mesh: Vec<Triangle<'mm>> = Vec::with_capacity(index_buffer.len() / 3);

        for i in 0..index_buffer.len() / 3 {
            triangle_mesh.push(Triangle::new(
                ObjectProperties { position: object_properties.position, material: object_properties.material },
                [
                    vertex_buffer[index_buffer[i * 3] as usize],
                    vertex_buffer[index_buffer[i * 3 + 1] as usize],
                    vertex_buffer[index_buffer[i * 3 + 2] as usize],
                ],
                cull_mode,
            ));
        }
        Self { object_properties, triangle_mesh }
    }
}

impl<'mm> Object<'mm> for TriangleMesh<'mm> {
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord<'mm>, is_shadow_ray: bool) -> bool {
        let mut hit_anything = false;

        for triangle in &self.triangle_mesh {
            if triangle.hit(ray, hit_record, is_shadow_ray) {
                hit_anything = true;
            }
        }

        hit_anything
    }

    fn update(&self, _delta_time: f32) {}
}
