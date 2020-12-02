mod dielectric;
mod libertian;
mod metal;

pub use {dielectric::Dielectric, libertian::Libertian, metal::Metal};

use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}
