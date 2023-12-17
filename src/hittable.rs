use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ops::Range;

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Range<f32>, depth: i32) -> Option<HitRecord> {
        None
    }
}
