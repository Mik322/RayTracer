use crate::camera::Camera;
use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::ray::ray_color;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn render_image(
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    camera: &Camera,
    world: &dyn Hittable,
) {
    let mut file = create_image(image_width, image_height);

    let mut rng = thread_rng();

    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut color = Color::zero();

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);
                color += ray_color(&r, world, max_depth);
            }
            write_color(&color, samples_per_pixel, &mut file);
        }
    }
}

fn create_image(image_width: i32, image_height: i32) -> File {
    let path = Path::new("img.ppm");
    match File::create(&path) {
        Err(e) => panic!("couldn't create {}: {}", path.display(), e),
        Ok(mut file) => {
            file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
                .expect("Could not write line");
            file
        }
    }
}
