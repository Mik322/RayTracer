pub mod libertian;
pub mod metal;

pub use libertian::Libertian;
pub use metal::Metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}
