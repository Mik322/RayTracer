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

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
