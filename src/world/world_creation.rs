use super::{scene_manager::SceneManager, scenegraph::Scenegraph};
use crate::{
    lights::{
        directional_light::DirectionalLight,
        light_manager,
        light_properties::{LightProperties, LightType},
        point_light::PointLight,
    },
    materials::material_manager::{self, MaterialManager, RoughnessConstants},
    objects::{
        object_properties::ObjectProperties,
        plane::Plane,
        sphere::Sphere,
        triangle::{CullMode, Triangle},
    },
};

use glam::Vec3;

pub fn create_materials(material_manager: &mut MaterialManager<'_>) {
    material_manager.add_lambert_material("Grey", 0);
    material_manager.add_lambert_material("Grey", 2);
    material_manager.add_lambert_material("Grey", 4);
    material_manager.add_lambert_material("Grey", 10);
    material_manager.add_lambert_material("White", 0);
    material_manager.add_lambert_material("Blue", 2);

    material_manager.add_lambert_phong_material("Blue", 0);
    material_manager.add_lambert_phong_material("Green", 0);
    material_manager.add_lambert_phong_material("Green", 4);
    material_manager.add_lambert_phong_material("Green", 10);

    // metals
    material_manager.add_phong_brdf_metal_material("Silver", RoughnessConstants::Smooth);
    material_manager.add_phong_brdf_metal_material("Silver", RoughnessConstants::HalfRough);
    material_manager.add_phong_brdf_metal_material("Silver", RoughnessConstants::Rough);
    material_manager.add_phong_brdf_metal_material("Gold", RoughnessConstants::Smooth);
    material_manager.add_phong_brdf_metal_material("Gold", RoughnessConstants::HalfRough);
    material_manager.add_phong_brdf_metal_material("Gold", RoughnessConstants::Rough);
    material_manager.add_phong_brdf_metal_material("Copper", RoughnessConstants::Smooth);

    // dielectrics
    material_manager.add_phong_brdf_dielectric_material("HotPink", RoughnessConstants::Smooth, 5);
    material_manager.add_phong_brdf_dielectric_material("SkyBlue", RoughnessConstants::Smooth, 5);
    material_manager.add_phong_brdf_dielectric_material("SkyBlue", RoughnessConstants::HalfRough, 3);
    material_manager.add_phong_brdf_dielectric_material("SkyBlue", RoughnessConstants::Rough, 2);
}

pub fn create_lights(light_manager: &mut light_manager::LightManager) {
    light_manager.add_light(Box::new(PointLight::new(
        LightProperties::new(Vec3::new(0.84, 0.8, 0.6), 100.0, true, LightType::Point),
        Vec3::new(-0.5, 5.5, 10.5),
    )));

    light_manager.add_light(Box::new(PointLight::new(
        LightProperties::new(Vec3::new(0.95, 0.65, 1.0), 50.0, true, LightType::Point),
        Vec3::new(0.3, 3.0, 10.5),
    )));

    light_manager.add_light(Box::new(PointLight::new(
        LightProperties::new(Vec3::new(0.55, 0.65, 1.0), 35.0, true, LightType::Point),
        Vec3::new(-0.2, 8.0, -2.),
    )));

    light_manager.add_light(Box::new(DirectionalLight::new(
        LightProperties::new(Vec3::new(0.8, 0.8, 0.8), 0.5, true, LightType::Directional),
        Vec3::new(0.0, -1.0, 0.0),
    )));
}

pub fn create_scenes<'a>(scene_manager: &mut SceneManager<'a>, material_manager: &'a MaterialManager<'a>) {
    create_scene_01(scene_manager, material_manager);
    create_scene_02(scene_manager, material_manager);
    //create_scene_03(&mut scene_manager);
    //create_scene_04(&mut scene_manager);
}

pub fn create_scene_01<'a>(scene_manager: &mut SceneManager<'a>, material_manager: &'a MaterialManager<'a>) {
    let mut scene = Scenegraph::new();

    let grey = material_manager.get_material("lambert_Grey_RE2").unwrap();
    let blue = material_manager.get_material("lambert_Blue_RE2").unwrap();
    let silver = material_manager.get_material("phong_brdf_Silver_metal_HalfRough").unwrap();
    let silver_smooth = material_manager.get_material("phong_brdf_Silver_metal_Smooth").unwrap();

    scene.add_object(Box::new(Sphere::new(ObjectProperties::new(Vec3::new(-1.0, 4.0, 0.0), silver_smooth), 1.0)));

    scene.add_object(Box::new(Sphere::new(ObjectProperties::new(Vec3::new(1.0, 4.0, 0.0), silver), 1.0)));

    scene.add_object(Box::new(Plane::new(ObjectProperties::new(Vec3::new(0.0, 0.0, 0.0), grey), Vec3::new(0.0, 1.0, 0.0))));

    scene.add_object(Box::new(Plane::new(ObjectProperties::new(Vec3::new(0.0, 0.0, -6.0), grey), Vec3::new(0.0, 0.0, 1.0))));

    scene.add_object(Box::new(Triangle::new(
        ObjectProperties::new(Vec3::new(-2.5, 6.0, 0.0), blue),
        [Vec3::new(-0.75, 1.5, 0.0), Vec3::new(-0.75, 0.0, 0.0), Vec3::new(0.75, 0.0, 0.0)],
        CullMode::None,
    )));

    scene_manager.add_scene(scene);
}

pub fn create_scene_02<'a>(scene_manager: &mut SceneManager<'a>, material_manager: &'a MaterialManager<'a>) {
    let mut scene = Scenegraph::new();

    let grey = material_manager.get_material("lambert_Grey_RE2").unwrap();
    let sky_blue_smooth = material_manager.get_material("phong_brdf_SkyBlue_dielectric_Smooth_RE5").unwrap();
    let hot_pink_smooth = material_manager.get_material("phong_brdf_HotPink_dielectric_Smooth_RE5").unwrap();

    scene.add_object(Box::new(Sphere::new(ObjectProperties::new(Vec3::new(-1.0, 4.0, 0.0), sky_blue_smooth), 1.0)));

    scene.add_object(Box::new(Sphere::new(ObjectProperties::new(Vec3::new(1.0, 4.0, 0.0), hot_pink_smooth), 1.0)));

    scene.add_object(Box::new(Plane::new(ObjectProperties::new(Vec3::new(0.0, 0.0, 0.0), grey), Vec3::new(0.0, 1.0, 0.0))));

    scene.add_object(Box::new(Plane::new(ObjectProperties::new(Vec3::new(0.0, 0.0, -6.0), grey), Vec3::new(0.0, 0.0, 1.0))));

    //add triangle
    scene.add_object(Box::new(Triangle::new(
        ObjectProperties::new(Vec3::new(-2.5, 6.0, 0.0), sky_blue_smooth),
        [Vec3::new(-0.75, 1.5, 0.0), Vec3::new(-0.75, 0.0, 0.0), Vec3::new(0.75, 0.0, 0.0)],
        CullMode::None,
    )));
    scene_manager.add_scene(scene);
}
