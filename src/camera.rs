use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn create(height: f64, aspect_ratio: f64, focal_len: f64) -> Camera {
        let origin = Vec3::zero();
        let horizontal = Vec3::new(height * aspect_ratio, 0.0, 0.0);
        let vertical = Vec3::new(0.0, height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_len);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::create(
            self.origin.clone(),
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
