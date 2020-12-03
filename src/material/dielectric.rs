use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};
use rand::prelude::*;

/// Represents a material that lets light go through it
#[derive(Clone, Copy)]
pub struct Dielectric {
    /// The index of refraction of the material
    ir: f64,
}

impl Dielectric {
    /// Returns a Dielectric material
    ///
    /// # Arguments
    ///
    /// * ir - index of refraction to give to the material
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    /// Returns the reflectance of the material when hit with a ray in a certain angle
    ///
    /// # Arguments
    ///
    /// * cosine - The cosine of the angle between the ray that hitted the material and the normal
    /// * ref_idx - The ratio between the materials index of refraction and the outside index of refraction
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(&r_in.dir);
        let cos_theta = Vec3::dot(&(-unit_direction), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction;
        if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > thread_rng().gen::<f64>()
        {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        let scattered = Ray::create(rec.p.clone(), direction);
        Some((scattered, attenuation))
    }
}

/// Returns the refractod vector from the incidence of the incoming vector agains a material
///
/// # Arguments
///
/// * uv - The vector that represents the direction of the incoming ray
/// * n - The normal of the plane hitted
/// * etai_over_etat - Ratio between the refraction index of the material the ray was traveling through and the material of the hitted object
fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = Vec3::dot(&(-uv), n).min(1.0);
    let r_out_perp: Vec3 = (uv + &(n * cos_theta)) * etai_over_etat;
    let r_out_parallel: Vec3 = n * -(1.0 - r_out_perp.lenght_squared()).abs().sqrt();
    r_out_parallel + r_out_perp
}
