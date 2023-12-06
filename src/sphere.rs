use crate::hittable::*;
use crate::vec3::Vec3;
use crate::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(self, center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = r.direction().length().sqrt();
        let half_b: f32 = Vec3::dot(&oc, &r.direction());
        let c: f32 = oc.length().sqrt() - self.radius * self.radius;

        let discriminant: f32 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd: f32 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root: f32 = (half_b - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            let root: f32 = (half_b + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        };

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
    }
}
