use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn create(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
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

        let mut rec = HitRecord::empty();

        rec.t = root;
        rec.p = ray.at(root);
        let outward_normal = (&rec.p - &self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }
}
