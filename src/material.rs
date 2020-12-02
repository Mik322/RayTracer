use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait material {
    fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &Color) -> Option<Ray>;
}
