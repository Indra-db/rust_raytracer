#![allow(dead_code)]
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
use glam::{IVec2, Vec3};
use lights::{
    light_manager::{Axis, LightManager},
    LightEnum,
};
use materials::material_manager::MaterialManager;

use renderer::Renderer;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton};

use world::{
    scene_manager::SceneManager,
    world_creation::{create_lights, create_materials, create_scenes},
};

fn print_key_mapping() {
    println!(
        "\n\n Raytracing in rust \n\n\n\
              Moving: WASD\n\
              Rotating: Hold Left Mouse Button\n\n\n\
              ---- The following keys correspond to the corresponding actions ----\n\n\
              Z: Turn off shadow casting \n\
              C: Change the amount of bounces (1,2,3,4,5,6 bounces) \n\
              V: Change Render modes (irradiance only, BRDF only and all) \n\n\
              P: Cycle through the lights in the scene to choose which one to select \n\
              O: Switch between 'Change Color' or 'Change Position' for the selected light \n\
              Y: Go to previous scene\n\
              U: Go to next scene\n\
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
    create_materials(&mut material_manager);

    let mut light_manager: LightManager<LightEnum> = LightManager::new();
    create_lights(&mut light_manager);

    let mut scene_manager: SceneManager<'_> = SceneManager::new();
    create_scenes(&mut scene_manager, &material_manager);

    let mut render_system = Renderer::new(width, height);
    let mut prev_mouse_x = 0;
    let mut prev_mouse_y = 0;
    let mut should_print_fps = false;

    print_key_mapping();

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
                    Keycode::Y => scene_manager.previous_scene(),
                    Keycode::U => scene_manager.next_scene(),
                    Keycode::Z => render_system.toggle_shadows(),
                    Keycode::V => render_system.toggle_render_mode(),
                    Keycode::C => render_system.toggle_max_bounce_rays(),
                    Keycode::P => light_manager.next_selected_light(),
                    Keycode::O => light_manager.change_interaction_mode(),
                    Keycode::Num0 => light_manager.toggle_selected_light(),
                    Keycode::Num1 => light_manager.change_value_of_interaction_mode(Axis::X, -0.04),
                    Keycode::Num2 => light_manager.change_value_of_interaction_mode(Axis::X, 0.04),
                    Keycode::Num3 => light_manager.change_value_of_interaction_mode(Axis::Y, -0.04),
                    Keycode::Num4 => light_manager.change_value_of_interaction_mode(Axis::Y, 0.04),
                    Keycode::Num5 => light_manager.change_value_of_interaction_mode(Axis::Z, -0.04),
                    Keycode::Num6 => light_manager.change_value_of_interaction_mode(Axis::Z, 0.04),
                    Keycode::Num7 => light_manager.change_intensity_of_selected_light(-0.50),
                    Keycode::Num8 => light_manager.change_intensity_of_selected_light(0.50),
                    Keycode::M => print_key_mapping(),
                    Keycode::N => should_print_fps = !should_print_fps,
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

        scene_manager.update(delta_time);

        render_system.render(
            canvas.get_pixel_data_mut(),
            scene_manager.get_current_scene(),
            &camera,
            light_manager.get_lights(),
        );

        canvas.flush();

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
                if should_print_fps {
                    println!("FPS: {frame_count}");
                }
                frame_count = 0;
                last_fps_time = current_time;
            }
        }
    }
}
