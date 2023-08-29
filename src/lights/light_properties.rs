use enum_dispatch::enum_dispatch;
use glam::Vec3;
pub type RGBColor = Vec3;

#[derive(Clone, PartialEq, Eq)]
pub enum LightType {
    Point,
    Directional,
}

#[derive(Clone, PartialEq)]
pub struct LightProperties {
    pub color: RGBColor,
    pub intensity: f32,
    pub is_enabled: bool,
    pub light_type: LightType,
}

#[enum_dispatch]
pub trait Light: Sync + Send {
    fn get_bi_radians(&self, position: &Vec3) -> RGBColor;
    fn get_direction(&self, position: &Vec3) -> Vec3;
    fn get_direction_magnitude(&self, position: &Vec3, direction_magnitude: &mut f32) -> Vec3;
    fn get_position(&self) -> &Vec3;
    fn get_position_mut(&mut self) -> &mut Vec3;
    fn get_color_mut(&mut self) -> &mut RGBColor;
    fn get_intensity_mut(&mut self) -> &mut f32;
    fn toggle_light(&mut self);
    fn get_type_light(&self) -> LightType;
    fn is_light_enabled(&self) -> bool;
}

impl LightProperties {
    pub const fn new(
        color: RGBColor,
        intensity: f32,
        is_enabled: bool,
        light_type: LightType,
    ) -> Self {
        Self { color, intensity, is_enabled, light_type }
    }

    pub fn toggle(&mut self) {
        self.is_enabled = !self.is_enabled;
    }
}
