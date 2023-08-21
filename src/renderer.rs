use crate::camera::Camera;
use crate::hitrecord::HitRecord;
use crate::lights::light_properties::Light;
use crate::ray::Ray;
use crate::world::scenegraph::Scenegraph;
use glam::{Mat4, Vec2, Vec3, Vec4};
use sdl2::render::WindowCanvas;
pub type RGBColor = Vec3;
use crate::math::ColorTypeFunctionality;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::{Surface, SurfaceRef};
use sdl2::sys::{
    SDL_GetWindowSurface, SDL_LockSurface, SDL_MapRGB, SDL_Surface, SDL_UnlockSurface,
    SDL_UpdateWindowSurface,
};

pub struct Renderer<'mm> {
    sdl_window: &'mm mut WindowCanvas,
    front_buffer: *mut SDL_Surface,
    //front_buffer: sdl2::surface::Surface<'static>,
    back_buffer: sdl2::surface::Surface<'static>,
    back_buffer_pixels: *mut u32,
    ray: Ray,
    ray_hit_to_light: Ray,
    hit_record: HitRecord<'mm>,
    hit_record_shadow: HitRecord<'mm>,
    ray_ss_coords: Vec2,
    width: u32,
    height: u32,
    aspect_ratio: f32,
    max_bounces: u32,
    amount_bounces: u32,
    reflectiveness_env_mat_first_hit: f32,
}

impl<'mm> Renderer<'mm> {
    pub fn new(sdl_window: &'mm mut WindowCanvas) -> Self {
        let width = sdl_window.window().size().0;
        let height = sdl_window.window().size().1;
        let aspect_ratio = width as f32 / height as f32;

        //let screen_bits = PixelFormatEnum::ARGB8888.into_masks().unwrap();
        //let front_buffer = Surface::from_pixelmasks(width, height, screen_bits).unwrap();
        let front_buffer: *mut SDL_Surface =
            unsafe { SDL_GetWindowSurface(sdl_window.window_mut().raw()) };

        //let screen_bits = PixelFormatEnum::Index8.into_masks().unwrap();
        let mut back_buffer = Surface::new(width, height, PixelFormatEnum::ABGR8888).unwrap();

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

    pub fn render(
        &mut self,
        scenegraph: &Scenegraph<'mm>,
        camera: &Camera,
        lights: &Vec<Box<dyn Light>>,
    ) {
        unsafe {
            SDL_LockSurface(self.back_buffer.raw());
        }

        let mut pixel: Vec4;
        let camera_look_at: &Mat4 = &camera.look_at;
        let scale_factor = camera.get_scale_factor();
        //loop over all the pixels

        for y in 0..self.height {
            self.get_ray_world_coord_y(y, scale_factor);

            for x in 0..self.width {
                self.ray.origin = camera.position;
                self.amount_bounces = 0;

                self.get_ray_world_coord_x(x, scale_factor);

                pixel = Vec4::new(self.ray_ss_coords.x, self.ray_ss_coords.y, -1.0, 0.0);

                pixel = camera_look_at.mul_vec4(pixel);

                self.ray.direction = (pixel.truncate() - self.ray.origin).normalize();

                let mut final_color: RGBColor =
                    self.calculate_color(scenegraph, lights, self.amount_bounces);

                final_color.max_to_one();
                //println!("6");
                #[allow(clippy::cast_precision_loss)]
                unsafe {
                    *self.back_buffer_pixels.add((x + y * self.width) as usize) = SDL_MapRGB(
                        self.back_buffer.pixel_format().raw(),
                        (0.0 * 255.0) as u8,
                        (0.0 * 255.0) as u8,
                        (0.0 * 255.0) as u8,
                    );
                }
                //println!("7");
            }
        }
        unsafe {
            SDL_UnlockSurface(self.back_buffer.raw());
            self.back_buffer.blit(None, SurfaceRef::from_ll_mut(self.front_buffer), None).unwrap();
            SDL_UpdateWindowSurface(self.sdl_window.window().raw());
        }
        //update_window

        //self.sdl_window.set_draw_color(Color::RGB(0, 0, 0));
        //self.sdl_window.clear();
        //self.sdl_window.window_mut().
        self.sdl_window.present();
    }

    fn calculate_color(
        &mut self,
        scenegraph: &Scenegraph<'mm>,
        lights: &Vec<Box<dyn Light>>,
        current_amount_bounces: u32,
    ) -> RGBColor {
        let mut color = RGBColor::ZERO;

        if !scenegraph.hit(&mut self.ray, &mut self.hit_record, false)
            || self.amount_bounces >= self.max_bounces
        {
            return color;
        }

        if self.amount_bounces == 0 {
            self.reflectiveness_env_mat_first_hit =
                self.hit_record.material.unwrap().get_reflectiveness_environment();
        }

        let mut lambert_cosine_law;

        self.ray_hit_to_light.origin = self.hit_record.hitpoint;

        for light in lights {
            if !light.is_light_enabled() {
                continue;
            }

            let mut direction_magnitude_returned = 0.0;

            self.ray_hit_to_light.direction = light.get_direction_magnitude(
                &self.hit_record.hitpoint,
                &mut direction_magnitude_returned,
            );

            self.ray_hit_to_light.t_max = direction_magnitude_returned;

            scenegraph.hit(&mut self.ray_hit_to_light, &mut self.hit_record_shadow, true);

            lambert_cosine_law = self.hit_record.normal.dot(self.ray_hit_to_light.direction);

            if lambert_cosine_law < 0.0 {
                continue;
            }

            color +=
                self.get_color_mode_according_to_render_mode(light.as_ref(), lambert_cosine_law);
        }

        if self.hit_record.material.unwrap().get_reflectiveness_environment().eq(&0.0) {
            return color;
        } else {
            let reflect = self.ray.direction
                - self.hit_record.normal * (self.ray.direction.dot(self.hit_record.normal) * 2.0);

            self.ray.direction = reflect.normalize();
            self.ray.origin = self.hit_record.hitpoint;

            color += self.calculate_color(scenegraph, lights, current_amount_bounces + 1)
                * self.reflectiveness_env_mat_first_hit;
        }

        color
    }

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
            * self.hit_record.material.unwrap().shade(
                &self.hit_record,
                &light.get_direction(&self.hit_record.hitpoint),
                &self.ray.direction,
            )

        //todo!("not fully implemented yet for different render modes");
    }

    fn change_max_bounce_rays(&mut self) {
        self.max_bounces += 1;
        if self.max_bounces > 4 {
            self.max_bounces = 1;
        }
        println!("\n\n current max bounces: {}", self.max_bounces);
    }
}
