use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ops::Range;

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Material,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_front_face(&mut self, val: bool) {
        self.front_face = val
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: Range<f32>, rec: &mut HitRecord) -> bool {
        return false;
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        let front_face: bool = Vec3::dot(&r.direction(), &outward_normal) < 0.0;
        if front_face {
            self.normal = outward_normal
        } else {
            self.normal = -outward_normal
        };
    }
}
