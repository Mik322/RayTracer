mod dielectric;
mod lambertian;
mod metal;

pub use {dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

use crate::{color::Color, hittable::HitRecord, ray::Ray};

/// Represents a material that can scatter a incoming ray
pub trait Material {
    /// Returns the ray that is scattered by the material and it's color
    ///
    /// # Arguments
    ///
    /// * r_in - The incoming ray that will be scattered
    /// * rec - HitRecord containing the informations about the hit
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}
