mod hittable_list;
mod sphere;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::rc::Rc;

pub use {hittable_list::HittableList, sphere::Sphere};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn create(
        t: f64,
        p: Point3,
        material: &Rc<dyn Material>,
        outward_normal: &Vec3,
        ray: &Ray,
    ) -> HitRecord {
        let front_face = Vec3::dot(outward_normal, &ray.dir) < 0.0;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
        HitRecord {
            p,
            normal,
            material: Rc::clone(material),
            t,
            front_face,
        }
    }
}

/// Trait that represents a object that is hittable by a ray
pub trait Hittable {
    /// Returns a Some(HitRecord) if the ray hitted the object and None if it doesn't
    ///
    /// # Arguments
    ///
    /// * ray - The ray that was casted
    /// * t_min - minimum time
    /// * t_max - maximum time
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
