use rayon::prelude::*;
use sdl2::pixels::Color;
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, render::Texture, render::TextureCreator};
use std::cell::RefCell;
use std::{thread::sleep, time::Duration};
type Error = Box<dyn std::error::Error>;

pub struct Canvas {
    pub sdl_context: sdl2::Sdl,
    pub sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    creator: TextureCreator<sdl2::video::WindowContext>,
    texture: RefCell<Texture<'static>>,
    pub width: u32,
    pub height: u32,
}
impl Canvas {
    pub fn new(width: u32, height: u32) -> Result<Self, Error> {
        let sdl_context = sdl2::init()?;
        sdl_context.mouse().set_relative_mouse_mode(true);
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window("Raytracing in Rust", width, height).position_centered().opengl().build()?;
        let mut sdl_canvas = window.into_canvas().build()?;
        sdl_canvas.set_draw_color(Color::RGB(0, 0, 0));
        sdl_canvas.clear();
        sdl_canvas.present();
        let creator = sdl_canvas.texture_creator();
        let texture = creator.create_texture_target(PixelFormatEnum::BGRA32, width, height)?;

        let texture = unsafe { std::mem::transmute::<_, Texture<'static>>(texture) };

        Ok(Canvas { width, height, sdl_canvas, sdl_context, creator, texture: RefCell::new(texture) })
    }

    pub fn flush(&mut self, raw_pixel_data: *const u8, length: usize) {
        let raw_pixel_data = unsafe { std::slice::from_raw_parts(raw_pixel_data, length * 4) };
        let mut texture = self.texture.borrow_mut();
        texture.update(None, raw_pixel_data, (self.width * 4) as usize).unwrap();
        self.sdl_canvas.copy(&texture, None, None).unwrap();
        self.sdl_canvas.present();
    }
}
