mod camera;
mod color;
mod hittable;
mod ray;
mod renderer;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::HittableList;
use rand::prelude::*;
use renderer::render_image;
use sphere::Sphere;
pub use std::f64::{consts::PI, MAX};
use vec3::Point3;

pub fn degress_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_float(min: f64, max: f64) -> f64 {
    min + (max - min) * thread_rng().gen::<f64>()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

//Next: 8

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let max_depth = 50;

    //World
    let mut world = HittableList::new();
    world.add(Sphere::create(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::create(Point3::new(0.0, -100.5, -1.0), 100.0));

    //Camera
    let camera = Camera::create(2.0, aspect_ratio, 1.0);

    render_image(image_width, image_height, 100, max_depth, &camera, &world);
}
