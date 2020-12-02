use crate::clamp;
pub use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color(color: &Color, samples_per_pixel: i32, file: &mut File) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let color_str = format!(
        "{} {} {}\n",
        (255.999 * clamp(r, 0.0, 0.9999)) as i32,
        (255.999 * clamp(g, 0.0, 0.9999)) as i32,
        (255.999 * clamp(b, 0.0, 0.9999)) as i32
    );
    file.write_all(color_str.as_bytes())
        .expect("Error in writing color");
}
