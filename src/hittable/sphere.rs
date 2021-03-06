use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::rc::Rc;

/// Represents a sphere that has a center, a radius and a material
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    /// Creates and returns a sphere with the gigen center, radius and material
    pub fn create(center: Point3, radius: f64, material: impl Material + 'static) -> Sphere {
        Sphere {
            center,
            radius,
            material: Rc::new(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.origin - &self.center;
        let a = ray.dir.lenght_squared();
        let half_b = Vec3::dot(&oc, &ray.dir);
        let c = oc.lenght_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        };

        let p = ray.at(root);
        let outward_normal = (&p - &self.center) / self.radius;

        let rec = HitRecord::create(root, p, &self.material, &outward_normal, ray);

        Some(rec)
    }
}
