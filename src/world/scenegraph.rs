use crate::hitrecord::HitRecord;
use crate::objects::object_properties::Object;
use crate::objects::ObjectEnum;
use crate::ray::Ray;

pub struct Scenegraph<'mm> {
    objects: Vec<ObjectEnum<'mm>>,
}

impl<'mm> Scenegraph<'mm> {
    pub const fn new() -> Self {
        Scenegraph { objects: Vec::new() }
    }

    pub fn add_object(&mut self, object: ObjectEnum<'mm>) {
        self.objects.push(object);
    }

    pub fn remove_object(&mut self, object: &ObjectEnum<'mm>) {
        if let Some(index) = self.objects.iter().position(|o| std::ptr::eq(o, object)) {
            self.objects.remove(index);
        }
    }

    pub const fn get_objects(&self) -> &Vec<ObjectEnum<'mm>> {
        &self.objects
    }

    pub fn update(&mut self, delta_time: f32) {
        for object in &mut self.objects {
            object.update(delta_time);
        }
    }

    pub fn hit(&self, ray: &mut Ray, hit_record: &mut HitRecord<'mm>, is_shadow: bool) -> bool {
        // Reset the t value of the hit record to the maximum value of f32 every new frame check
        if !is_shadow {
            ray.t_max = f32::MAX;
        }

        self.objects.iter().fold(false, |hit_anything, object| {
            if object.hit(ray, hit_record, is_shadow) {
                ray.t_max = hit_record.t;
                true
            } else {
                hit_anything
            }
        })
    }
}
