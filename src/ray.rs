use crate::color::Color;
use crate::hittable::Hittable;
use crate::vec3::{Point3, Vec3};
use crate::MAX;

/// Represents a ray that can be casted from an origin Point3 and has a direction
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    ///Returns a ray catested from a given point in a given direction
    ///
    /// # Arguments
    ///
    /// * origin - Point from where the ray was casted
    /// * dir - The direction of the ray
    pub fn create(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    /// Returns the point the ray reached after a given time
    ///
    /// # Arguments
    ///
    /// * time - The time passed since the ray was casted
    pub fn at(&self, time: f64) -> Point3 {
        let v = self.clone();
        v.origin + v.dir * time
    }
}

/// Returns the color of the casted ray by detecting the color of the hitted objects
///
/// # Arguments
///
/// * ray - The ray to detect it's color
/// * world - The hittable_list that contains all the hittables the ray may hit
/// * depth - Maximum number of refractions/reflections a ray can have
pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth == 0 {
        return Color::zero();
    }
    let mut color = Color::new(1.0, 1.0, 1.0);
    let mut r = ray.clone();
    for _ in 0..depth {
        match world.hit(&r, 0.001, MAX) {
            Some(rec) => match rec.material.scatter(&r, &rec) {
                Some((scattered, attenuation)) => {
                    r = scattered;
                    color = color * attenuation;
                }
                None => {
                    color = Color::zero();
                    break;
                }
            },
            None => {
                let unit_dir = Vec3::unit_vector(&r.dir);
                let t = 0.5 * (unit_dir.y + 1.0);
                color =
                    color * (Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t);
                break;
            }
        }
    }

    color
}
