use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

/// Struct that contains a vector of hittables
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    /// Returns an empty HittableList
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    /// Adds an hittable object to the HittablesList vector
    pub fn add(&mut self, obj: impl Hittable + 'static) {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for hittable in &self.objects {
            if let Some(temp_rec) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }

        rec
    }
}
