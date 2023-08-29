use crate::hitrecord::HitRecord;
use crate::lights::light_properties::Light;
use crate::materials::material_properties::Material;
use crate::math::ColorTypeFunctionality;
use crate::ray::Ray;
use crate::world::scenegraph::Scenegraph;
use crate::{camera::Camera, lights::LightEnum};
use glam::{Mat4, Vec2, Vec3, Vec4};
use rayon::prelude::*;
pub type RGBColor = Vec3;

pub struct Renderer {
    aspect_ratio: f32,
    width: u32,
    height: u32,
    max_bounces: u32,
    pub are_hard_shadows_enabled: bool,
    pub render_mode: u8,
}

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        let max_bounces = 4;
        Self {
            aspect_ratio,
            width,
            height,
            max_bounces,
            are_hard_shadows_enabled: true,
            render_mode: 0,
        }
    }

    pub fn render(
        &self,
        pixel_data: &mut Vec<u32>,
        scenegraph: &Scenegraph<'_>,
        camera: &Camera,
        lights: &Vec<LightEnum>,
    ) {
        let camera_look_at: &Mat4 = &camera.look_at;
        let scale_factor = camera.get_scale_factor();

        // We are grabbing a parallel iterator over rows
        pixel_data.par_chunks_mut(self.width as usize).enumerate().for_each(|(y, row)| {
            let mut ray_ss_coords: Vec2 =
                Vec2 { x: 0.0, y: self.get_ray_world_coord_y(y as u32, scale_factor) };

            for (x, pixel_data) in row.iter_mut().enumerate() {
                let mut ray = Ray::new(camera.position, Vec3::ZERO);
                ray_ss_coords.x = self.get_ray_world_coord_x(x as u32, scale_factor);

                let pixel =
                    *camera_look_at * Vec4::new(ray_ss_coords.x, ray_ss_coords.y, -1.0, 1.0);

                ray.direction = (pixel.truncate() - ray.origin).normalize();

                let mut final_color: RGBColor =
                    self.calculate_color(scenegraph, lights, 0, &mut ray);
                final_color.max_to_one();

                let final_color = Self::to_u32_rgb(final_color.x, final_color.y, final_color.z);
                *pixel_data = final_color;
            }
        });
    }

    fn to_u32_rgb(r: f32, g: f32, b: f32) -> u32 {
        let ri = (r * 255.0) as u32;
        let gi = (g * 255.0) as u32;
        let bi = (b * 255.0) as u32;

        (ri << 16) | (gi << 8) | bi
    }

    fn calculate_color(
        &self,
        scenegraph: &Scenegraph<'_>,
        lights: &Vec<LightEnum>,
        mut current_amount_bounces: u32,
        ray: &mut Ray,
    ) -> RGBColor {
        let mut accumulated_color = RGBColor::ZERO;
        let mut reflectiveness_env_mat_first_hit = 1.0;
        let mut previous_reflectivness = 1.0;
        let mut previous_lambert = 1.0;

        while current_amount_bounces < self.max_bounces {
            let mut bounce_color = RGBColor::ZERO;

            let mut hit_record = HitRecord::default();
            if !scenegraph.hit(ray, &mut hit_record, false) {
                break;
            }

            if current_amount_bounces == 0 {
                reflectiveness_env_mat_first_hit = match hit_record.material {
                    Some(_) => hit_record.material.unwrap().get_reflectiveness_environment(),
                    None => return RGBColor::ZERO,
                };
            }

            let mut lambert_cosine_law;
            let offset = 0.0001;

            let mut ray_hit_to_light = Ray {
                origin: hit_record.hitpoint + (hit_record.normal * offset),
                ..Default::default()
            };

            for light in lights {
                if !light.is_light_enabled() {
                    continue;
                }

                if self.are_hard_shadows_enabled {
                    let mut direction_magnitude_returned = 0.0;

                    ray_hit_to_light.direction = light.get_direction_magnitude(
                        &hit_record.hitpoint,
                        &mut direction_magnitude_returned,
                    );

                    ray_hit_to_light.t_min = 0.0001;
                    ray_hit_to_light.t_max = direction_magnitude_returned;
                    let mut hit_record_shadow = HitRecord::default();
                    if scenegraph.hit(&mut ray_hit_to_light, &mut hit_record_shadow, true) {
                        continue;
                    }
                }

                lambert_cosine_law =
                    hit_record.normal.dot(light.get_direction(&hit_record.hitpoint));

                if lambert_cosine_law < 0.0 {
                    continue;
                }

                bounce_color += self.get_color_mode_according_to_render_mode(
                    light,
                    lambert_cosine_law,
                    &hit_record,
                    ray,
                );
            }

            if hit_record.material.unwrap().get_reflectiveness_environment().eq(&0.0) {
                break;
            }

            let reflect =
                ray.direction - hit_record.normal * (ray.direction.dot(hit_record.normal) * 2.0);

            lambert_cosine_law = hit_record.normal.dot(reflect);

            ray.direction = reflect.normalize();
            ray.origin = hit_record.hitpoint;

            accumulated_color += bounce_color * previous_reflectivness * previous_lambert;

            previous_reflectivness = reflectiveness_env_mat_first_hit;
            previous_lambert = lambert_cosine_law;

            current_amount_bounces += 1;
        }
        //accumulated_color = accumulated_color / lights.len() as f32;
        accumulated_color
    }

    fn get_ray_world_coord_x(&self, x: u32, scale_factor: f32) -> f32 {
        2.0f32.mul_add((x as f32 + 0.5) / self.width as f32, -1.0)
            * self.aspect_ratio
            * scale_factor
    }

    fn get_ray_world_coord_y(&self, y: u32, scale_factor: f32) -> f32 {
        2.0f32.mul_add(-(y as f32 + 0.5) / self.height as f32, 1.0) * scale_factor
    }

    fn get_color_mode_according_to_render_mode(
        &self,
        light: &LightEnum,
        lambert_cosine_law: f32,
        hit_record: &HitRecord<'_>,
        ray: &Ray,
    ) -> RGBColor {
        match self.render_mode {
            0 => {
                light.get_bi_radians(&hit_record.hitpoint)
                    * lambert_cosine_law
                    * hit_record.material.unwrap().shade(
                        hit_record,
                        &light.get_direction(&hit_record.hitpoint),
                        &(-1.0 * ray.direction),
                    )
            }
            1 => light.get_bi_radians(&hit_record.hitpoint) * lambert_cosine_law,
            2 => {
                lambert_cosine_law
                    * hit_record.material.unwrap().shade(
                        hit_record,
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
}
