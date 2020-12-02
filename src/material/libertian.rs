use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub struct Libertian {
    albedo: Color,
}

impl Libertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Material for Libertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = &rec.normal + &Vec3::random_unit_vec();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        let scattered = Ray::create(rec.p.clone(), scatter_direction);
        Some((scattered, self.albedo))
    }
}
