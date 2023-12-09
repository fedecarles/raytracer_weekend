use camera::Camera;
use hittable_list::*;
use ray::Ray;
use sphere::*;
use vec3::Vec3;

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;

// Utilities
fn degrees_to_radian(degrees: f32) -> f32 {
    return degrees * std::f32::consts::PI / 180.0;
}

fn main() {
    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam: Camera = Camera::default();
    cam.initialize();
    cam.samples_per_pixel = 100;
    cam.max_deph = 50;
    cam.render(&world)
}
