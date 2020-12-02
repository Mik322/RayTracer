use crate::color::Color;
use crate::hittable::Hittable;
use crate::vec3::{Point3, Vec3};
use crate::MAX;

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn create(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, time: f64) -> Point3 {
        let v = self.clone();
        v.origin + v.dir * time
    }
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zero();
    };

    if let Some(rec) = world.hit(ray, 0.001, MAX) {
        if let Some((scattered, attenuation)) = rec.material.scatter(ray, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::zero();
    }

    let unit_dir = Vec3::unit_vector(&ray.dir);
    let t = 0.5 * (unit_dir.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
