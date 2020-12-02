use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(&r_in.dir, &rec.normal);
        let scattered = Ray::create(rec.p.clone(), reflected);

        if Vec3::dot(&scattered.dir, &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
