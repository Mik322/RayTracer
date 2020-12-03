use crate::{
    degress_to_radians,
    ray::Ray,
    vec3::{Point3, Vec3},
};

/// Represents a camera object
pub struct Camera {
    /// Origin point that represents the camera's position
    origin: Point3,
    /// Vector that represents the horizontal length
    horizontal: Vec3,
    /// Vector that represents the vertical length
    vertical: Vec3,
    /// Position of the lower left corner
    lower_left_corner: Vec3,
    /// Unitary vector that represents the camera's x axis
    u: Vec3,
    /// Unitary vector that represents the camera's y axis
    v: Vec3,
    /// Half of the lens' aperture
    lens_radius: f64,
}

impl Camera {
    /// Creates a camera and returns it
    ///
    /// # Arguments
    ///
    /// * lookfrom - position of the camera
    /// * lookat - point where the camera is looking at
    /// * vup - vector representing the up orientation
    /// * vfov - vertical field of view in degress
    /// * aspect_ratio - camera's desired aspect ratio (width/height)
    /// * aperture - radius of the lens
    /// * focus_dist - the distance between the sensor and the lens
    pub fn create(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degress_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(&(lookfrom - lookat));
        let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    /// Returns a ray casted from the camera's lens
    ///
    /// # Arguments
    ///
    /// * s - horizontal factor, it's multiplied by horizontal vector to get the ray horizontal direction
    /// * t - vertical factor, it's multiplied by vertical factor to get the ray vertical direction
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;

        Ray::create(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
