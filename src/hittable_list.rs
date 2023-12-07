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
    fn hit(&self, r: Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.set_t(closest_so_far);
                rec.set_p(temp_rec.p());
                rec.set_face_normal(&r, temp_rec.normal);
            }
        }
        return hit_anything;
    }
}
