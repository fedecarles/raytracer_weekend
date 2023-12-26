use crate::hittable::*;
use crate::hittable_list::HittableList;
use crate::material::scatter;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::ops::Range;

pub fn color(r: &Ray, depth: i32, world: &HittableList) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(
        r,
        Range {
            start: 0.001,
            end: std::f32::INFINITY,
        },
        depth,
    ) {
        let mut scattered = Ray::ray(Vec3::default(), Vec3::default());
        let mut attenuation = Vec3::default();

        if scatter(&rec.material, &r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * color(&scattered, depth - 1, &world);
        } else {
            return Vec3::default();
        }
    } else {
        let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
        let a: f32 = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}

pub fn reflectance(cosine: f32, ir: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - ir) / (1.0 + ir)).powi(2);

    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
