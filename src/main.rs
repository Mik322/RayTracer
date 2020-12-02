mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod renderer;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::{HittableList, Sphere};
use material::{Dielectric, Libertian, Metal};
use rand::prelude::*;
use renderer::render_image;
pub use std::f64::{consts::PI, MAX};
use vec3::{Point3, Vec3};

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

fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    //World
    let world = random_scene();

    //Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;

    let camera = Camera::create(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    render_image(
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        &camera,
        &world,
    );
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Libertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::create(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));

    let mut rng = thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.0, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let sphere_material = Libertian::new(albedo);
                    world.add(Sphere::create(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuz = random_float(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuz);
                    world.add(Sphere::create(center, 0.2, sphere_material));
                } else {
                    //glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere::create(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    let material2 = Libertian::new(Color::new(0.4, 0.2, 0.1));
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    world.add(Sphere::create(Point3::new(0.0, 1.0, 0.0), 1.0, material1));
    world.add(Sphere::create(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));
    world.add(Sphere::create(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}
