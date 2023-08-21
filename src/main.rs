#![allow(dead_code)]
#![allow(unused_imports)]
//#![deny(rust_2018_idioms)]
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
mod hitrecord;
mod lights;
mod materials;
mod math;
mod objects;
mod ray;
mod renderer;
mod world;

use glam::{IVec2, Mat4, Vec3};
use lights::light_manager::{self, LightManager};
use lights::light_properties::{LightProperties, LightType};
use lights::point_light::PointLight;
use materials::material_manager::{self, MaterialManager};
use objects::object_properties::ObjectProperties;
use objects::sphere::Sphere;
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
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Raytracing in Rust", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut frame_count = 0u32;
    let mut last_fps_time;
    unsafe {
        last_fps_time = sdl2::sys::SDL_GetPerformanceCounter();
    }

    let mut camera = camera::Camera::new(Vec3::new(0.0, 2.0, 15.0), 45.0);
    let mut scene: Scenegraph = Scenegraph::new();

    let mut delta_time: f32 = 0.001;
    let mut material_manager: MaterialManager = MaterialManager::new();
    material_manager.add_lambert_material("Grey", 0);
    let mut light_manager: LightManager = LightManager::new();
    light_manager.add_light(Box::new(PointLight::new(
        LightProperties::new(Vec3::new(0.84, 0.8, 0.6), 100.0, true, LightType::Point),
        Vec3::new(-0.5, 5.5, 6.5),
    )));

    scene.add_object(Box::new(Sphere::new(
        ObjectProperties::new(
            Vec3::new(-1.0, 4.0, 0.0),
            material_manager.get_material("Grey").unwrap(),
        ),
        1.0,
    )));

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
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        camera.camera_rotation(delta_time, IVec2::new(x, y));
                    }
                }
                _ => {}
            }
        }

        camera.update_look_at();

        frame_count += 1;
        unsafe {
            let current_time = sdl2::sys::SDL_GetPerformanceCounter();
            #[allow(clippy::cast_precision_loss)]
            let elapsed_seconds = (current_time - last_fps_time) as f32
                / sdl2::sys::SDL_GetPerformanceFrequency() as f32;
            delta_time = elapsed_seconds;
            if elapsed_seconds >= 1.0 {
                println!("FPS: {frame_count}");
                frame_count = 0;
                last_fps_time = current_time;
            }
        }
    }

    drop(scene);
}
