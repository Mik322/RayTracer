use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub struct Metal {
    albedo: Color,
    fuz: f64,
}

impl Metal {
    pub fn new(color: Color, fuz: f64) -> Self {
        let fuz = if fuz < 1.0 { fuz } else { 1.0 };
        Self { albedo: color, fuz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(&r_in.dir, &rec.normal);
        let scattered = Ray::create(
            rec.p.clone(),
            reflected + Vec3::random_in_unit_sphere() * self.fuz,
        );

        if Vec3::dot(&scattered.dir, &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
