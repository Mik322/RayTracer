use std::{fs::File, io::Write, path::Path};

use crate::camera::Camera;
use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::ray::ray_color;
use rand::prelude::*;

pub fn render_image(
    image_width: i32,
    image_height: i32,
    image_name: String,
    samples_per_pixel: i32,
    max_depth: i32,
    camera: &Camera,
    world: &dyn Hittable,
) {
    let f = create_image(image_width, image_height, image_name);

    let mut rng = thread_rng();

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut color = Color::zero();

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);
                color += ray_color(&r, world, max_depth);
            }
            write_color(&color, samples_per_pixel, &f);
        }
    }
}

fn create_image(image_width: i32, image_height: i32, image_name: String) -> File {
    let file_name = format!("{}.ppm", image_name);
    let path = Path::new(&file_name);
    match File::create(&path) {
        Err(e) => panic!("couldn't create {}: {}", path.display(), e),
        Ok(mut file) => {
            file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
                .expect("Could not write line");
            file
        }
    }
}
