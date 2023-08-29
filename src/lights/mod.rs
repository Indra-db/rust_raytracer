pub mod directional_light;
pub mod light_manager;
pub mod light_properties;
pub mod point_light;

pub use directional_light::DirectionalLight;
use enum_dispatch::enum_dispatch;
use glam::Vec3;
pub use light_properties::{Light, LightProperties, LightType};
pub use point_light::PointLight;
pub type RGBColor = Vec3;

#[enum_dispatch(Light)]
pub enum LightEnum {
    Point(PointLight),
    Directional(DirectionalLight),
}
