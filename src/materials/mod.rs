pub mod lambert_material;
pub mod lambert_phong_material;
pub mod material_definitions;
pub mod material_manager;
pub mod material_properties;
pub mod phong_brdf_material;

use super::hitrecord::HitRecord;
use enum_dispatch::enum_dispatch;
use glam::Vec3;
use lambert_material::LambertMaterial;
use lambert_phong_material::LambertPhongMaterial;
use material_properties::Material;
use phong_brdf_material::PhongBRDFMaterial;
type RGBColor = Vec3;

#[enum_dispatch(Material)]
pub enum MaterialEnum {
    Lambert(LambertMaterial),
    LambertPhong(LambertPhongMaterial),
    PhongBRDFMaterial(PhongBRDFMaterial),
}
