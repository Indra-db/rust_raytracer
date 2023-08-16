#![allow(dead_code)]
#![allow(unused_imports)]
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

mod math;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

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

    let window =
        video_subsystem.window("Raytracing in Rust", 800, 600).position_centered().build().unwrap();

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

    'running: loop {
        frame_count += 1;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                _ => {}
            }
        }

        unsafe {
            let current_time = sdl2::sys::SDL_GetPerformanceCounter();
            #[allow(clippy::cast_precision_loss)]
            let elapsed_seconds = (current_time - last_fps_time) as f64
                / sdl2::sys::SDL_GetPerformanceFrequency() as f64;

            if elapsed_seconds >= 1.0 {
                println!("FPS: {frame_count}");
                frame_count = 0;
                last_fps_time = current_time;
            }
        }
    }
}
