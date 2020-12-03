use crate::random_float;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

/// Represents a vector with 3 dimensions
#[derive(Clone, Copy)]
pub struct Vec3 {
    /// A Vec3 contains the 3 cordinates (x, y, z) as f64
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
/// Point3 represents a point in 3d space, is an Vec3 alias
pub type Point3 = Vec3;

impl Vec3 {
    /// Returns a Vec3 with the given cordinates
    ///
    /// # Arguments
    ///
    /// * x
    /// * y
    /// * z
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Returns the length of the Vec3
    pub fn length(&self) -> f64 {
        self.lenght_squared().sqrt()
    }

    /// Veturns true if the length of the vector is less than 1e-8
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x < s && self.y < s && self.z < s
    }

    /// Returns the result of the mathematical operation dot between two vectors
    ///
    /// # Arguments
    ///
    /// * v1 - Reference to a Vec3
    /// * v2 - Referenco to a Vec3
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    /// Returns the normal vector of the plane represented by the two given vectors
    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    /// Returns the vector that was reflected from an incoming vector
    ///
    /// # Arguments
    ///
    /// * v - The vector that will be reflected
    /// * normal - The normal of the plane the given vector hitted
    pub fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
        v - &(normal * Vec3::dot(v, normal) * 2.0)
    }

    /// Returns the normalized vector
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        let vec = v.clone();
        return vec / vec.length();
    }

    pub fn lenght_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(normal, &in_unit_sphere) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_unit_vec() -> Vec3 {
        Vec3::unit_vector(&Vec3::random_in_unit_sphere())
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p;
        loop {
            p = Vec3::random(-1.0, 1.0);
            if p.lenght_squared() < 1.0 {
                break;
            }
        }
        p
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut p = Vec3::new(1.0, 1.0, 0.0);
        loop {
            p.x = random_float(-1.0, 1.0);
            p.y = random_float(-1.0, 1.0);
            if p.lenght_squared() < 1.0 {
                break;
            }
        }
        p
    }
    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_float(min, max),
            y: random_float(min, max),
            z: random_float(min, max),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn add(self, other: &'b Vec3) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'b Vec3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<'a> Mul<f64> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<'a> Neg for &'a Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<isize> for Vec3 {
    type Output = f64;

    fn index(&self, i: isize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
