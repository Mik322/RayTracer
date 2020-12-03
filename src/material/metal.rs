use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

/// Represents a material with metal properties
pub struct Metal {
    /// The coler of the metal
    albedo: Color,
    /// The fuzzines of the metal, the more it has the less light reflects
    fuzz: f64,
}

impl Metal {
    /// Returns a metal material with the given color and fuziness
    pub fn new(color: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self {
            albedo: color,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(&r_in.dir, &rec.normal);
        let scattered = Ray::create(
            rec.p.clone(),
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
        );

        if Vec3::dot(&scattered.dir, &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
