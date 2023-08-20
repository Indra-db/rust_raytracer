use super::light_properties::{Light, LightProperties, LightType};
use glam::Vec3;
pub type RGBColor = Vec3;

#[derive(Clone, PartialEq)]
pub struct PointLight {
    pub light_properties: LightProperties,
    position: Vec3,
}

impl PointLight {
    pub const fn new(light_properties: LightProperties, position: Vec3) -> Self {
        Self { light_properties, position }
    }
}

impl Light for PointLight {
    fn get_bi_radians(&self, position: &Vec3) -> RGBColor {
        let direction_sq = self.position.distance_squared(*position);
        self.light_properties.color * self.light_properties.intensity / direction_sq
    }

    fn get_direction(&self, position: &Vec3) -> Vec3 {
        (self.position - *position).normalize()
    }

    fn get_direction_magnitude(&self, position: &Vec3, direction_magnitude: &mut f32) -> Vec3 {
        let normalized_direction = (self.position - *position).normalize();
        *direction_magnitude = normalized_direction.length();
        normalized_direction
    }

    fn get_position(&self) -> &Vec3 {
        &self.position
    }

    fn get_position_mut(&mut self) -> &mut Vec3 {
        &mut self.position
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
}
