use camera::Camera;
use hittable::*;
use hittable_list::*;
use material::{scatter, Material};
use ray::Ray;
use sphere::*;
use std::ops::Range;
use vec3::Vec3;

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

// Utilities
fn degrees_to_radian(degrees: f32) -> f32 {
    return degrees * std::f32::consts::PI / 180.0;
}

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

fn main() {
    // World
    let mut world = HittableList::default();

    let material_ground = Material::Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    };
    let material_center = Material::Lambertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    };
    let material_left = Material::Metal {
        albedo: Vec3::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    };
    let material_right = Material::Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let mut cam: Camera = Camera::default();
    cam.initialize();
    cam.samples_per_pixel = 100;
    cam.max_deph = 50;
    cam.render(&world)
}
