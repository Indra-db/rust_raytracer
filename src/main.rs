#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(rust_2018_idioms)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::correctness,
    clippy::suspicious
)]

mod camera;
mod canvas;
mod hitrecord;
mod lights;
mod materials;
mod math;
mod objects;
mod ray;
mod renderer;
mod world;

use canvas::Canvas;
use glam::{IVec2, Mat4, Vec3};
use lights::directional_light::DirectionalLight;
use lights::light_manager::{self, LightManager};
use lights::light_properties::{LightProperties, LightType};
use lights::point_light::PointLight;
use materials::lambert_material;
use materials::lambert_material::LambertianMaterial;
use materials::material_manager::{self, MaterialManager};
use materials::material_properties::{Material, MaterialProperties, RGBColor};
use objects::object_properties::ObjectProperties;
use objects::plane::Plane;
use objects::sphere::Sphere;
use renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use std::rc::Rc;
use std::time::Duration;
use world::scenegraph::Scenegraph;

fn print_key_mapping() {
    println!(
        "\n\n Raytracing in rust \n\n\n\
              Moving: WASD\n\
              Rotating: Hold Left Mouse Button\n\n\n\
              ---- The following keys correspond to the corresponding actions ----\n\n\
              Z: Turn off shadow casting \n\
              X: Screenshot (saved in solution folder) \n\
              C: Change the amount of bounces (1,2,3,4 bounces) \n\
              V: Change Render modes (irradiance only, BRDF only and all) \n\n\
              P: Cycle through the lights in the scene to choose which one to select \n\
              O: Switch between 'Change Color' or 'Change Position' for the selected light \n\
              Y: Go to previous scene\n\
              U: Go to next scene\n\
              I: Cycle through the cube maps loaded in \n\
              0: Turn off selected light \n\n\
              1 & 2: Change x/r value of the selected light pos/color \n\
              3 & 4: Change y/g value of the selected light pos/color \n\
              5 & 6: Change z/b value of the selected light pos/color \n\
              7 & 8: Change intensity of the selected light \n\n\
              M: Print key mapping again \n\
              N: Print FPS \n\n"
    );
}

fn main() {
    let width = 800;
    let height = 600;
    let mut canvas = Canvas::new(width, height).unwrap();
    let mut event_pump = canvas.sdl_context.event_pump().unwrap();

    let mut frame_count = 0u32;
    let mut last_fps_time;
    let mut previous_time;

    unsafe {
        last_fps_time = sdl2::sys::SDL_GetPerformanceCounter();
    }

    previous_time = last_fps_time;

    let mut camera = camera::Camera::new(Vec3::new(0.0, 2.0, 15.0), 45.0);

    let mut delta_time: f32 = 0.001;

    let mut material_manager: MaterialManager<'_> = MaterialManager::new();
    material_manager.add_lambert_material("Grey", 0);
    material_manager.add_lambert_phong_material("Grey", 0);
    material_manager.add_phong_brdf_material(
        "Silver",
        true,
        material_manager::RoughnessConstants::Smooth,
        0,
    );

    let mut light_manager: LightManager = LightManager::new();

    light_manager.add_light(Box::new(PointLight::new(
        LightProperties::new(Vec3::new(0.84, 0.8, 0.6), 100.0, true, LightType::Point),
        Vec3::new(-0.5, 5.5, 6.5),
    )));

    light_manager.add_light(Box::new(PointLight::new(
        LightProperties::new(Vec3::new(0.95, 0.65, 1.0), 100.0, true, LightType::Point),
        Vec3::new(0.3, 3.0, 6.5),
    )));

    light_manager.add_light(Box::new(PointLight::new(
        LightProperties::new(Vec3::new(0.55, 0.65, 1.0), 100.0, true, LightType::Point),
        Vec3::new(-0.2, 8.0, -5.),
    )));

    light_manager.add_light(Box::new(DirectionalLight::new(
        LightProperties::new(Vec3::new(0.6, 0.35, 0.3), 0.8, true, LightType::Directional),
        Vec3::new(0.0, -1.0, 0.0),
    )));

    let mut scene: Scenegraph<'_> = Scenegraph::new();

    let grey_01 = material_manager.get_material("lambert_Grey_RE0").unwrap();
    let silver = material_manager.get_material("phong_brdf_Silver_Metal_Smooth").unwrap();

    scene.add_object(Box::new(Sphere::new(
        ObjectProperties::new(Vec3::new(-1.0, 4.0, 0.0), grey_01),
        1.0,
    )));

    scene.add_object(Box::new(Sphere::new(
        ObjectProperties::new(Vec3::new(1.0, 4.0, 0.0), silver),
        1.0,
    )));

    scene.add_object(Box::new(Plane::new(
        ObjectProperties::new(Vec3::new(0.0, 0.0, 0.0), grey_01),
        Vec3::new(0.0, 1.0, 0.0),
    )));

    scene.add_object(Box::new(Plane::new(
        ObjectProperties::new(Vec3::new(0.0, 0.0, -6.0), grey_01),
        Vec3::new(0.0, 0.0, 1.0),
    )));

    let mut render_system = Renderer::new(&mut canvas);
    let mut prev_mouse_x = 0;
    let mut prev_mouse_y = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                Event::KeyDown { keycode: Some(key), .. } => match key {
                    Keycode::W | Keycode::S | Keycode::A | Keycode::D | Keycode::Q | Keycode::E => {
                        camera.camera_translation(delta_time, key);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let mouse_state = event_pump.relative_mouse_state();
        let (mouse_x, mouse_y) = (mouse_state.x(), mouse_state.y());
        if mouse_state.is_mouse_button_pressed(MouseButton::Left)
            && mouse_x != prev_mouse_x
            && mouse_y != prev_mouse_y
        {
            camera.camera_rotation(delta_time, IVec2::new(mouse_x, mouse_y));
            prev_mouse_x = mouse_x;
            prev_mouse_y = mouse_y;
        }

        camera.update_look_at();

        scene.update(delta_time);

        render_system.render(&scene, &camera, light_manager.get_lights());

        frame_count += 1;

        #[allow(clippy::cast_precision_loss)]
        unsafe {
            let current_time = sdl2::sys::SDL_GetPerformanceCounter();
            delta_time = (current_time - previous_time) as f32
                / sdl2::sys::SDL_GetPerformanceFrequency() as f32;
            previous_time = current_time;
            let elapsed_seconds = (current_time - last_fps_time) as f32
                / sdl2::sys::SDL_GetPerformanceFrequency() as f32;

            if elapsed_seconds >= 1.0 {
                println!("FPS: {frame_count}");
                frame_count = 0;
                last_fps_time = current_time;
            }
        }
    }
}
