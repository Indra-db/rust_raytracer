use super::light_properties::{Light, LightProperties, LightType};
use glam::Vec3;
pub type RGBColor = Vec3;

#[derive(Clone, PartialEq)]
pub struct DirectionalLight {
    pub light_properties: LightProperties,
    direction: Vec3,
}

impl DirectionalLight {
    pub const fn new(light_properties: LightProperties, direction: Vec3) -> Self {
        Self { light_properties, direction }
    }
}

impl Light for DirectionalLight {
    fn get_bi_radians(&self, _position: &Vec3) -> RGBColor {
        self.light_properties.color * self.light_properties.intensity
    }

    fn get_direction(&self, _position: &Vec3) -> Vec3 {
        -self.direction
    }

    fn get_direction_magnitude(&self, _position: &Vec3, direction_magnitude: &mut f32) -> Vec3 {
        *direction_magnitude = std::f32::INFINITY;
        -self.direction
    }

    fn get_position(&self) -> &Vec3 {
        &self.direction
    }

    fn get_position_mut(&mut self) -> &mut Vec3 {
        &mut self.direction
    }

    fn get_color_mut(&mut self) -> &mut RGBColor {
        &mut self.light_properties.color
    }

    fn get_intensity_mut(&mut self) -> &mut f32 {
        &mut self.light_properties.intensity
    }

    fn toggle_light(&mut self) {
        self.light_properties.toggle();
    }

    fn get_type_light(&self) -> LightType {
        self.light_properties.light_type.clone()
    }

    fn is_light_enabled(&self) -> bool {
        self.light_properties.is_enabled
    }
}
