use crate::hitrecord::HitRecord;
use crate::lights::light_properties::Light;
use crate::ray::Ray;
use glam::{Vec2, Vec3};
pub type RGBColor = Vec3;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

pub struct Renderer {
    sdl_window: sdl2::video::Window,
    front_buffer: sdl2::surface::Surface<'static>,
    back_buffer: sdl2::surface::Surface<'static>,
    back_buffer_pixels: *mut u32,
    ray: Ray,
    ray_hit_to_light: Ray,
    hit_record: HitRecord,
    hit_record_shadow: HitRecord,
    ray_ss_coords: Vec2,
    width: u32,
    height: u32,
    aspect_ratio: f32,
    max_bounces: u32,
    amount_bounces: u32,
    reflectiveness_env_mat_first_hit: f32,
}

impl Renderer {
    pub fn new(sdl_window: sdl2::video::Window) -> Self {
        let width = sdl_window.size().0;
        let height = sdl_window.size().1;
        let aspect_ratio = width as f32 / height as f32;

        let screen_bits = PixelFormatEnum::ARGB8888.into_masks().unwrap();
        let front_buffer = Surface::from_pixelmasks(width, height, screen_bits).unwrap();

        let screen_bits = PixelFormatEnum::Index8.into_masks().unwrap();
        let mut back_buffer = Surface::from_pixelmasks(width, height, screen_bits).unwrap();

        let back_buffer_pixels = back_buffer.without_lock_mut().unwrap().as_mut_ptr() as *mut u32;
        let ray = Ray::new(Vec3::ZERO, Vec3::ZERO);
        let ray_hit_to_light = Ray::new(Vec3::ZERO, Vec3::ZERO);
        let hit_record = HitRecord::default();
        let hit_record_shadow = HitRecord::default();
        let ray_ss_coords = Vec2::ZERO;
        let max_bounces = 4;
        let amount_bounces = 0;
        let reflectiveness_env_mat_first_hit = 0.0;
        Self {
            sdl_window,
            front_buffer,
            back_buffer,
            back_buffer_pixels,
            ray,
            ray_hit_to_light,
            hit_record,
            hit_record_shadow,
            ray_ss_coords,
            width,
            height,
            aspect_ratio,
            max_bounces,
            amount_bounces,
            reflectiveness_env_mat_first_hit,
        }
    }

    pub fn render() {}

    fn get_ray_world_coord_x(&mut self, x: u32, scale_factor: f32) {
        self.ray_ss_coords.x = ((2.0 * ((x as f32 + 0.5) / self.width as f32)) - 1.0)
            * self.aspect_ratio
            * scale_factor;
    }

    fn get_ray_world_coord_y(&mut self, y: u32, scale_factor: f32) {
        self.ray_ss_coords.y = 1.0 - (2.0 * ((y as f32 + 0.5) / self.height as f32)) * scale_factor;
    }

    fn get_color_mode_according_to_render_mode(
        &self,
        light: &dyn Light,
        lambert_cosine_law: f32,
    ) -> RGBColor {
        light.get_bi_radians(&self.hit_record.hitpoint)
            * lambert_cosine_law
            * self.hit_record.material.shade(
                &self.hit_record,
                &light.get_direction(&self.hit_record.hitpoint),
                &self.ray.direction,
            );

        todo!("not fully implemented yet for different render modes");
    }

    fn change_max_bounce_rays(&mut self) {
        self.max_bounces += 1;
        if (self.max_bounces > 4) {
            self.max_bounces = 1;
        }
        println!("\n\n current max bounces: {}", self.max_bounces);
    }
}
