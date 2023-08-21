use crate::hitrecord::HitRecord;
use crate::objects::object_properties::Object;
use crate::ray::Ray;

pub struct Scenegraph<'mm> {
    objects: Vec<Box<dyn Object<'mm>>>,
}

impl<'mm> Scenegraph<'mm> {
    pub fn new() -> Self {
        Scenegraph { objects: Vec::new() }
    }

    pub fn add_object(&mut self, object: Box<dyn Object<'mm>>) {
        self.objects.push(object);
    }

    pub fn remove_object(&mut self, object: &Box<dyn Object<'mm>>) {
        if let Some(index) = self.objects.iter().position(|o| o as *const _ == object as *const _) {
            self.objects.remove(index);
        }
    }

    pub fn get_objects(&self) -> &Vec<Box<dyn Object<'mm>>> {
        &self.objects
    }

    pub fn update(&mut self, delta_time: f32) {
        for object in &mut self.objects {
            object.update(delta_time);
        }
    }

    pub fn hit(&self, ray: &mut Ray, hit_record: &mut HitRecord<'mm>, is_shadow: bool) -> bool {
        let mut hit_anything = false;

        //we reset the t value of the hit record to the maximum value of f32 every new frame check
        if !is_shadow {
            ray.t_max = f32::MAX;
        }

        for object in &self.objects {
            if object.hit(ray, hit_record, is_shadow) {
                hit_anything = true;
                ////change t_max to know in which order we have to render the objects
                ray.t_max = hit_record.t;
            }
        }

        hit_anything
    }
}
