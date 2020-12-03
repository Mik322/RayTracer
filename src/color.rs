use crate::clamp;
pub use crate::vec3::Vec3;
pub type Color = Vec3;
use std::{fs::File, io::Write};

/// Writes to a file the color of the pixel
///
/// # Arguments
///
/// * color - The sum of the colors to be written in the pixel
/// * samples_per_pixel - Amount of rays that were cast for this pixel
/// * file - PPM file that will be writen the pixel coler
pub fn write_color(color: &Color, samples_per_pixel: i32, mut file: &File) {
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
