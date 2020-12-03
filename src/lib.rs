//! # Ray_Tracer
//!
//! A ray tracer written in rust following the [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).
//! This crate also took inspiration from the ray tracer develop by [_gkmngrgn_](https://github.com/gkmngrgn)

pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod renderer;
pub mod vec3;

use rand::prelude::*;
pub use std::f64::{consts::PI, MAX};

/// Converts a number in degrees to radians
pub fn degress_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Returns a random float between the min amnd max given
pub fn random_float(min: f64, max: f64) -> f64 {
    min + (max - min) * thread_rng().gen::<f64>()
}

/// Return x if it's in the range between min and max, if it's over returns max and if it's under returns min
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
