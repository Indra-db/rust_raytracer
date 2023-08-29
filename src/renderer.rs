use crate::camera::Camera;
use crate::hitrecord::HitRecord;
use crate::lights::light_properties::Light;
use crate::ray::Ray;
use crate::world::scenegraph::Scenegraph;
use glam::{Mat4, Vec2, Vec3, Vec4};
use sdl2::render::WindowCanvas;
pub type RGBColor = Vec3;
use crate::canvas::Canvas;
use crate::math::ColorTypeFunctionality;
use rayon::prelude::*;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::{Surface, SurfaceRef};
pub struct Renderer {
    aspect_ratio: f32,
    width: u32,
    height: u32,
    max_bounces: u32,
    pub are_hard_shadows_enabled: bool,
    pub render_mode: u8,
    pub data: Vec<u32>,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        let max_bounces = 2;
        Self {
            aspect_ratio,
            width,
            height,
            max_bounces,
            are_hard_shadows_enabled: true,
            render_mode: 0,
            data: vec![0; (width * height) as usize],
        }
    }

    pub fn render(&mut self, scenegraph: &Scenegraph<'_>, camera: &Camera, lights: &Vec<Box<dyn Light + Sync>>) {
        let camera_look_at: &Mat4 = &camera.look_at;
        let scale_factor = camera.get_scale_factor();

        // We are grabbing a parallel iterator over rows
        self.data.par_chunks_mut(self.width as usize).into_par_iter().enumerate().for_each(|(y, row)| {
            for x in 0..self.width {
                let mut ray = Ray::new(camera.position, Vec3::ZERO);
                let mut ray_ss_coords: Vec2 = Vec2::ZERO;

                ray_ss_coords.x = Renderer::get_ray_world_coord_y(y as u32, scale_factor, self.height);
                ray_ss_coords.y = Renderer::get_ray_world_coord_x(x, scale_factor, self.width, self.aspect_ratio);

                let pixel = *camera_look_at * Vec4::new(ray_ss_coords.x, ray_ss_coords.y, -1.0, 1.0);
                ray.direction = (pixel.truncate() - ray.origin).normalize();

                let mut reflectiveness_env_mat_first_hit = 0.0;
                //let mut final_color: RGBColor = RGBColor::ZERO;
                let mut final_color: RGBColor = Renderer::calculate_color(
                    self.are_hard_shadows_enabled,
                    self.render_mode,
                    self.max_bounces,
                    scenegraph,
                    lights,
                    0,
                    &mut ray,
                    &mut reflectiveness_env_mat_first_hit,
                );

                final_color.max_to_one();

                let final_color = Renderer::to_u32_rgb(final_color.x, final_color.y, final_color.z);
                row[x as usize] = final_color;
            }
        });

        //            let mut final_color: RGBColor = self.calculate_color(scenegraph, lights, 0);
        //
        //            final_color.max_to_one();
        //
        //            let final_color = Renderer::to_u32_rgb(final_color.x, final_color.y, final_color.z);
        //
        //            self.canvas.draw_pixel(x, y, final_color);
        //        }
        //    }
        //});
    }

    fn to_u32_rgb(r: f32, g: f32, b: f32) -> u32 {
        let ri = (r * 255.0) as u32;
        let gi = (g * 255.0) as u32;
        let bi = (b * 255.0) as u32;

        (ri << 16) | (gi << 8) | bi
    }

    fn calculate_color(
        are_hard_shadows_enabled: bool,
        render_mode: u8,
        max_bounces: u32,
        scenegraph: &Scenegraph<'_>,
        lights: &Vec<Box<dyn Light + Sync>>,
        current_amount_bounces: u32,
        ray: &mut Ray,
        reflectiveness_env_mat_first_hit: &mut f32,
    ) -> RGBColor {
        let mut color = RGBColor::ZERO;

        if current_amount_bounces >= max_bounces {
            return color;
        }
        let mut hit_record = HitRecord::default();
        if !scenegraph.hit(ray, &mut hit_record, false) {
            return color;
        }

        if current_amount_bounces == 0 {
            *reflectiveness_env_mat_first_hit = match hit_record.material {
                Some(_) => hit_record.material.unwrap().get_reflectiveness_environment(),
                None => return RGBColor::ZERO,
            };
        }

        let mut lambert_cosine_law;
        let offset = 0.0001;
        let mut ray_hit_to_light = Ray::default();
        ray_hit_to_light.origin = hit_record.hitpoint + (hit_record.normal * offset);

        for light in lights {
            if !light.is_light_enabled() {
                continue;
            }

            if are_hard_shadows_enabled {
                let mut direction_magnitude_returned = 0.0;

                ray_hit_to_light.direction =
                    light.get_direction_magnitude(&hit_record.hitpoint, &mut direction_magnitude_returned);

                ray_hit_to_light.t_min = 0.0001; //-> f32::EPSILON is too low.
                ray_hit_to_light.t_max = direction_magnitude_returned;
                let mut hit_record_shadow = HitRecord::default();
                if scenegraph.hit(&mut ray_hit_to_light, &mut hit_record_shadow, true) {
                    continue;
                }
            }

            lambert_cosine_law = hit_record.normal.dot(light.get_direction(&hit_record.hitpoint));

            if lambert_cosine_law < 0.0 {
                continue;
            }

            color += Renderer::get_color_mode_according_to_render_mode(
                render_mode,
                light.as_ref(),
                lambert_cosine_law,
                &hit_record,
                ray,
            );
        }

        //color = color / lights.len() as f32; //this currently makes everything too dark

        if hit_record.material.unwrap().get_reflectiveness_environment().eq(&0.0) {
            return color;
        } else {
            let reflect = ray.direction - hit_record.normal * (ray.direction.dot(hit_record.normal) * 2.0);
            lambert_cosine_law = hit_record.normal.dot(reflect);

            ray.direction = reflect.normalize();
            ray.origin = hit_record.hitpoint;

            color += Renderer::calculate_color(
                are_hard_shadows_enabled,
                render_mode,
                max_bounces,
                scenegraph,
                lights,
                current_amount_bounces + 1,
                ray,
                reflectiveness_env_mat_first_hit,
            ) * *reflectiveness_env_mat_first_hit
                * lambert_cosine_law;
        }

        color
    }

    fn get_ray_world_coord_x(x: u32, scale_factor: f32, width: u32, aspect_ratio: f32) -> f32 {
        ((2.0 * ((x as f32 + 0.5) / width as f32)) - 1.0) * aspect_ratio * scale_factor
    }

    fn get_ray_world_coord_y(y: u32, scale_factor: f32, height: u32) -> f32 {
        (1.0 - (2.0 * ((y as f32 + 0.5) / height as f32))) * scale_factor
    }

    fn get_color_mode_according_to_render_mode(
        render_mode: u8,
        light: &dyn Light,
        lambert_cosine_law: f32,
        hit_record: &HitRecord<'_>,
        ray: &Ray,
    ) -> RGBColor {
        match render_mode {
            0 => {
                light.get_bi_radians(&hit_record.hitpoint)
                    * lambert_cosine_law
                    * hit_record.material.unwrap().shade(
                        &hit_record,
                        &light.get_direction(&hit_record.hitpoint),
                        &(-1.0 * ray.direction),
                    )
            }
            1 => light.get_bi_radians(&hit_record.hitpoint) * lambert_cosine_law,
            2 => {
                lambert_cosine_law
                    * hit_record.material.unwrap().shade(
                        &hit_record,
                        &light.get_direction(&hit_record.hitpoint),
                        &(-1.0 * ray.direction),
                    )
            }
            _ => RGBColor::ZERO,
        }
    }

    pub fn toggle_max_bounce_rays(&mut self) {
        self.max_bounces += 1;
        if self.max_bounces > 6 {
            self.max_bounces = 1;
        }
        println!("\n\n current max bounces: {}", self.max_bounces);
    }

    pub fn toggle_shadows(&mut self) {
        self.are_hard_shadows_enabled = !self.are_hard_shadows_enabled;
    }

    pub fn toggle_render_mode(&mut self) {
        self.render_mode += 1;

        match self.render_mode {
            0 => println!("Render mode: all"),
            1 => println!("Render mode: biradiance"),
            2 => println!("Render mode: BRDF"),
            _ => self.render_mode = 0,
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        self.data[(y * self.width + x) as usize] = color;
    }
    pub fn data_raw(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4) }
    }

    /// Returns a parallel iterator over mutable rows of the canvas.
    pub fn par_rows_mut(&mut self) -> impl ParallelIterator<Item = &mut [u32]> {
        self.data.par_chunks_mut(self.width as usize).into_par_iter()
    }

    pub fn get_pixel_data_raw(&self) -> (usize, *const u8) {
        (self.data.len(), self.data.as_ptr() as *const u8)
    }
}
