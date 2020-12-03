pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod renderer;
pub mod vec3;

use rand::prelude::*;
pub use std::f64::{consts::PI, MAX};

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
