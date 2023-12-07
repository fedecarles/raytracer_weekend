use hittable::*;
use hittable_list::*;
use ray::Ray;
use sphere::*;
use std::ops::Range;
use vec3::Vec3;

pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn color(r: Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(
        r,
        Range {
            start: 0.0,
            end: std::f32::INFINITY,
        },
        &mut rec,
    ) {
        return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
    }
    let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
    let a: f32 = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}

fn write_color(pixel_color: Vec3) {
    println!(
        "{} {} {}",
        (255.99 * pixel_color.x()) as i32,
        (255.99 * pixel_color.y()) as i32,
        (255.99 * pixel_color.z()) as i32
    )
}

fn main() {
    // image size
    let aspect_ratio: f32 = 16.0 / 9.0;
    let width: i32 = 400;
    let height: i32 = std::cmp::max((width as f32 / aspect_ratio) as i32, 1);

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length: f32 = 1.0;
    let camera_center: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    // viewport size
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (width as f32 / height as f32);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Vec3 = viewport_u / width as f32;
    let pixel_delta_v: Vec3 = viewport_v / height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left: Vec3 =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_00_loc: Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let max_value: i32 = 255;

    // Render
    println!("P3\n{} {}\n{}", width, height, max_value);
    for j in (0..height).rev() {
        for i in 0..width {
            let pixel_center: Vec3 =
                pixel_00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction: Vec3 = pixel_center - camera_center;

            let r: Ray = Ray::ray(camera_center, ray_direction);
            let color = color(r, &world);

            write_color(color);
        }
    }
}
