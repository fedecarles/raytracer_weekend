use std::ops::Range;

use crate::hittable::*;
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { objects }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Range<f32>, depth: i32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far: f32 = ray_t.end;

        for object in &self.objects {
            if let Some(rec) = object.hit(
                &r,
                Range {
                    start: ray_t.start,
                    end: closest_so_far,
                },
                depth,
            ) {
                closest_so_far = rec.t;
                hit_record = Some(rec)
            }
        }
        return hit_record;
    }
}
