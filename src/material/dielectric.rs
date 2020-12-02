use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};
use rand::prelude::*;

#[derive(Clone, Copy)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

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

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = Vec3::dot(&(-uv), n).min(1.0);
    let r_out_perp: Vec3 = (uv + &(n * cos_theta)) * etai_over_etat;
    let r_out_parallel: Vec3 = n * -(1.0 - r_out_perp.lenght_squared()).abs().sqrt();
    r_out_parallel + r_out_perp
}
