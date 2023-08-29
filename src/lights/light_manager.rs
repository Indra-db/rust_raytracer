use super::light_properties::Light;
use glam::Vec3;
use std::boxed::Box;

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub struct LightManager {
    lights: Vec<Box<dyn Light + Sync>>,
    selected_light_index: usize,
}

impl LightManager {
    pub fn new() -> Self {
        Self { lights: Vec::new(), selected_light_index: 0 }
    }

    pub fn add_light(&mut self, light: Box<dyn Light + Sync>) {
        self.lights.push(light);
    }

    pub fn remove_light(&mut self, light: &Box<dyn Light + Sync>) {
        if let Some(index) = self.lights.iter().position(|l| std::ptr::eq(l, light)) {
            self.lights.remove(index);
        }
    }

    pub fn next_selected_light(&mut self) {
        self.selected_light_index = (self.selected_light_index + 1) % self.lights.len();
    }

    pub fn toggle_selected_light(&mut self) {
        self.lights[self.selected_light_index].toggle_light();

        todo!("not safe")
    }

    pub fn change_pos_selected_light(&mut self, axis: Axis, value: f32) {
        let light = &mut self.lights[self.selected_light_index];

        // Pattern match on the light type
        let pos: &mut Vec3 = light.get_position_mut();

        // Pattern match on the axis
        match axis {
            Axis::X => {
                pos.x += value;
                println!("selected light X pos: {}", pos.x);
            }
            Axis::Y => {
                pos.y += value;
                println!("selected light Y pos: {}", pos.y);
            }
            Axis::Z => {
                pos.z += value;
                println!("selected light Z pos: {}", pos.z);
            }
        }
    }

    pub fn change_color_of_selected_light(&mut self, axis: Axis, value: f32) {
        let light = &mut self.lights[self.selected_light_index];

        let color = light.get_color_mut();

        match axis {
            Axis::X => {
                color.x += value;
                println!("selected light X color: {}", color.x);
            }
            Axis::Y => {
                color.y += value;
                println!("selected light Y color: {}", color.y);
            }
            Axis::Z => {
                color.z += value;
                println!("selected light Z color: {}", color.z);
            }
        }
    }

    pub fn change_intensity_of_selected_light(&mut self, value: f32) {
        let light = &mut self.lights[self.selected_light_index];

        let intensity = light.get_intensity_mut();

        *intensity += value;
        println!("selected light intensity: {intensity}");
    }

    pub fn num_lights(&self) -> usize {
        self.lights.len()
    }

    pub fn get_lights(&self) -> &Vec<Box<dyn Light + Sync>> {
        &self.lights
    }
}
