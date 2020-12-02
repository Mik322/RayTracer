mod hittable_list;
mod sphere;

use crate::{
    color::Color,
    material::{Libertian, Material},
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
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(outward_normal, &ray.dir) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }

    pub fn empty() -> HitRecord {
        Self {
            p: Point3::zero(),
            normal: Vec3::zero(),
            material: Rc::new(Libertian::new(Color::zero())),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
