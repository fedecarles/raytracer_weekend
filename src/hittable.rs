use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool {
        return false;
    }
}
