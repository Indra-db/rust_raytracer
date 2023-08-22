use sdl2::pixels::Color;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::PixelFormatEnum, render::Texture,
    render::TextureCreator,
};
use std::cell::RefCell;
use std::{thread::sleep, time::Duration};

type Error = Box<dyn std::error::Error>;

pub struct Canvas {
    pub sdl_context: sdl2::Sdl,
    pub sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    creator: TextureCreator<sdl2::video::WindowContext>,
    texture: RefCell<Texture<'static>>,
    data: Vec<u32>,
    pub width: u32,
    pub height: u32,
}
impl Canvas {
    pub fn new(width: u32, height: u32) -> Result<Self, Error> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Raytracing in Rust", 800, 600)
            .position_centered()
            .opengl()
            .build()?;
        let mut sdl_canvas = window.into_canvas().build()?;
        sdl_canvas.set_draw_color(Color::RGB(0, 0, 0));
        sdl_canvas.clear();
        sdl_canvas.present();
        let creator = sdl_canvas.texture_creator();
        let texture = creator.create_texture_target(PixelFormatEnum::RGBA8888, width, height)?;

        let texture = unsafe { std::mem::transmute::<_, Texture<'static>>(texture) };

        Ok(Canvas {
            width,
            height,
            data: vec![0; (width * height) as usize],
            sdl_canvas,
            sdl_context,
            creator,
            texture: RefCell::new(texture),
        })
    }
    pub fn flush(&mut self) {
        let mut texture = self.texture.borrow_mut();
        texture.update(None, self.data_raw(), (self.width * 4) as usize).unwrap();
        self.sdl_canvas.copy(&texture, None, None).unwrap();
        self.sdl_canvas.present();
    }
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        self.data[(y * self.width + x) as usize] = color;
    }
    pub fn data_raw(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4) }
    }
}
