use std::ops::Range;

use crate::hittable::*;
use crate::material::Material;
use crate::vec3::Vec3;
use crate::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Range<f32>, _depth: i32) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = r.direction().length_squared();
        let half_b: f32 = Vec3::dot(&oc, &r.direction());
        let c: f32 = oc.length_squared() - self.radius.powi(2);

        let discriminant: f32 = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd: f32 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root: f32 = (-half_b - sqrtd) / a;
        if (&root <= &ray_t.start) || (&ray_t.end <= &root) {
            root = (-half_b + sqrtd) / a;
            if (&root <= &ray_t.start) || (&ray_t.end <= &root) {
                return None;
            }
        };

        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        let outward_normal = (r.at(root) - self.center) / self.radius;
        let front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        return Some(HitRecord {
            p: r.at(root),
            normal: normal,
            material: self.material,
            t: root,
            front_face: front_face,
        });
    }
}
