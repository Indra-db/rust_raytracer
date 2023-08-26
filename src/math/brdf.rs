use glam::Vec3;
pub type RGBColor = Vec3;

pub fn lambert(diffuse_color: &RGBColor, diffuse_reflectance: f32) -> RGBColor {
    (*diffuse_color * diffuse_reflectance) / std::f32::consts::PI
}

pub fn lambert_color(diffuse_color: &RGBColor, diffuse_reflectance: &RGBColor) -> RGBColor {
    (*diffuse_color * *diffuse_reflectance) / std::f32::consts::PI
}

pub fn phong(specular_reflectance_factor: f32, phong_exponent: i32, light_direction: &Vec3, view_direction: &Vec3, normal: &Vec3) -> RGBColor {
    let reflect: Vec3 = -*light_direction + 2.0 * normal.dot(*light_direction) * *normal;
    let cos_angle = reflect.dot(*view_direction);

    if cos_angle > 0.0 {
        let phong_specular_reflection = specular_reflectance_factor * cos_angle.powi(phong_exponent);
        RGBColor::new(phong_specular_reflection, phong_specular_reflection, phong_specular_reflection)
    } else {
        RGBColor::default()
    }
}

pub fn trowbridge_reitz_ggx(normal_surface: &Vec3, half_vector: &Vec3, roughness: f32) -> f32 {
    let roughness_pow4 = roughness.powi(4);
    let n_dot_h_sq = normal_surface.dot(*half_vector).max(f32::EPSILON).powi(2);

    let denom = std::f32::consts::PI * n_dot_h_sq.mul_add(roughness_pow4 - 1.0, 1.0).powi(2);
    roughness_pow4 / denom
}

pub fn schlick(half_vector: &Vec3, view_dir: &Vec3, base_reflectivity_surface: &RGBColor) -> RGBColor {
    let h_dot_v = half_vector.dot(*view_dir).max(f32::EPSILON);
    *base_reflectivity_surface + (RGBColor::ONE - *base_reflectivity_surface) * (1.0 - h_dot_v).powi(5)
}

/// a Fresnel function from Schlich that describes the reflectivity of the microfacets.
/// Returns a float describing the reflectivity of the microfacets
///
/// # Arguments
///
/// * `k` - the base reflectivity of the surface -> (0.04,0.04,0.04) for dielectrics or the albedo value for the metals.
/// * `halfVector` - the half vector between the view direction and the light direction
pub fn schlick_ggx(half_vector: &Vec3, view_dir: &Vec3, k: f32) -> f32 {
    let n_dot_v = half_vector.normalize().dot(view_dir.normalize()).max(f32::EPSILON);
    n_dot_v / n_dot_v.mul_add(1.0 - k, k)
}

/// Smith method: using the Schlick method for both the shadowing using the light direction
/// and the masking using the view direction and multiplying them
/// Returns a float that describes the overshadowing of microfacets
///
/// # Arguments
///
/// * `k` - the roughness reampped based on whether you use the function with direct or indirect lighting. (using direct lighting ATM).
pub fn smith_method(normal_surface: &Vec3, view_dir: &Vec3, light_dir: &Vec3, roughness: f32) -> f32 {
    schlick_ggx(normal_surface, view_dir, roughness) * schlick_ggx(normal_surface, light_dir, roughness)
}
