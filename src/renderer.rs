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
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::{Surface, SurfaceRef};

pub struct Renderer<'mm> {
    canvas: &'mm mut Canvas,
    ray: Ray,
    ray_hit_to_light: Ray,
    hit_record: HitRecord<'mm>,
    hit_record_shadow: HitRecord<'mm>,
    ray_ss_coords: Vec2,
    aspect_ratio: f32,
    max_bounces: u32,
    amount_bounces: u32,
    reflectiveness_env_mat_first_hit: f32,
}

impl<'mm> Renderer<'mm> {
    pub fn new(canvas: &'mm mut Canvas) -> Self {
        let aspect_ratio = canvas.width as f32 / canvas.height as f32;
        let ray = Ray::new(Vec3::ZERO, Vec3::ZERO);
        let ray_hit_to_light = Ray::new(Vec3::ZERO, Vec3::ZERO);
        let hit_record = HitRecord::default();
        let hit_record_shadow = HitRecord::default();
        let ray_ss_coords = Vec2::ZERO;
        let max_bounces = 4;
        let amount_bounces = 0;
        let reflectiveness_env_mat_first_hit = 0.0;
        Self {
            canvas,
            ray,
            ray_hit_to_light,
            hit_record,
            hit_record_shadow,
            ray_ss_coords,
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
        let mut pixel: Vec4;
        let camera_look_at: &Mat4 = &camera.look_at;
        let scale_factor = camera.get_scale_factor();

        for y in 0..self.canvas.height {
            self.get_ray_world_coord_y(y, scale_factor);

            for x in 0..self.canvas.width {
                self.ray.origin = camera.position;
                self.amount_bounces = 0;

                self.get_ray_world_coord_x(x, scale_factor);

                pixel = Vec4::new(self.ray_ss_coords.x, self.ray_ss_coords.y, -1.0, 1.0);

                pixel = *camera_look_at * pixel;

                self.ray.direction = (pixel.truncate() - self.ray.origin).normalize();

                let mut final_color: RGBColor =
                    self.calculate_color(scenegraph, lights, self.amount_bounces);

                final_color.max_to_one();

                let final_color = Renderer::to_u32_rgb(final_color.x, final_color.y, final_color.z);

                self.canvas.draw_pixel(x, y, final_color);
            }
        }

        self.canvas.flush();
    }

    fn to_u32_rgb(r: f32, g: f32, b: f32) -> u32 {
        let ri = (r * 255.0) as u32;
        let gi = (g * 255.0) as u32;
        let bi = (b * 255.0) as u32;

        (ri << 16) | (gi << 8) | bi
    }

    fn calculate_color(
        &mut self,
        scenegraph: &Scenegraph<'mm>,
        lights: &Vec<Box<dyn Light>>,
        current_amount_bounces: u32,
    ) -> RGBColor {
        let mut color = RGBColor::ZERO;

        if current_amount_bounces >= self.max_bounces {
            return color;
        }

        if !scenegraph.hit(&mut self.ray, &mut self.hit_record, false) {
            return color;
        }

        if self.amount_bounces == 0 {
            self.reflectiveness_env_mat_first_hit = match self.hit_record.material {
                Some(_) => self.hit_record.material.unwrap().get_reflectiveness_environment(),
                None => return RGBColor::ZERO,
            };
        }

        let mut lambert_cosine_law = 1.0;
        let offset = 0.0001;

        self.ray_hit_to_light.origin = self.hit_record.hitpoint + (self.hit_record.normal * offset);

        for light in lights {
            if !light.is_light_enabled() {
                continue;
            }

            let mut direction_magnitude_returned = 0.0;

            self.ray_hit_to_light.direction = light.get_direction_magnitude(
                &self.hit_record.hitpoint,
                &mut direction_magnitude_returned,
            );

            //self.ray_hit_to_light.t_min = 0.0; -> fucks everything up
            self.ray_hit_to_light.t_max = direction_magnitude_returned;

            if scenegraph.hit(&mut self.ray_hit_to_light, &mut self.hit_record_shadow, true) {
                continue;
            }

            lambert_cosine_law =
                self.hit_record.normal.dot(light.get_direction(&self.hit_record.hitpoint));

            if lambert_cosine_law < 0.0 {
                continue;
            }

            color +=
                self.get_color_mode_according_to_render_mode(light.as_ref(), lambert_cosine_law);
        }

        color = color / lights.len() as f32;
        if self.hit_record.material.unwrap().get_reflectiveness_environment().eq(&0.0) {
            return color;
        } else {
            let reflect = self.ray.direction
                - self.hit_record.normal * (self.ray.direction.dot(self.hit_record.normal) * 2.0);

            self.ray.direction = reflect.normalize();
            self.ray.origin = self.hit_record.hitpoint;

            color += self.calculate_color(scenegraph, lights, current_amount_bounces + 1)
                * self.reflectiveness_env_mat_first_hit
                * lambert_cosine_law;
        }

        color
    }

    fn get_ray_world_coord_x(&mut self, x: u32, scale_factor: f32) {
        self.ray_ss_coords.x = ((2.0 * ((x as f32 + 0.5) / self.canvas.width as f32)) - 1.0)
            * self.aspect_ratio
            * scale_factor;
    }

    fn get_ray_world_coord_y(&mut self, y: u32, scale_factor: f32) {
        self.ray_ss_coords.y =
            (1.0 - (2.0 * ((y as f32 + 0.5) / self.canvas.height as f32))) * scale_factor;
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
