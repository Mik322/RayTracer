use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

/// Represents a material that uses lambertian reflectance to scatter light
pub struct Lambertian {
    /// The color of the material
    albedo: Color,
}

impl Lambertian {
    /// Returns a Lambertian material with the given color
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = &rec.normal + &Vec3::random_unit_vec();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        let scattered = Ray::create(rec.p.clone(), scatter_direction);
        Some((scattered, self.albedo))
    }
}
