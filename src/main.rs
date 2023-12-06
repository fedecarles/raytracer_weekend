use ray::Ray;
use vec3::Vec3;

pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> f32 {
    let oc: Vec3 = r.origin() - center;
    let a: f32 = r.direction().length().sqrt();
    let half_b: f32 = Vec3::dot(&oc, &r.direction());
    let c: f32 = oc.length().sqrt() - radius * radius;
    let discriminant: f32 = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn color(r: &Ray) -> Vec3 {
    let t: f32 = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n: Vec3 = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
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

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    // Render
    println!("P3\n{} {}\n{}", width, height, max_value);
    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f32 / width as f32;
            let v = j as f32 / height as f32;
            //let r: Ray = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);

            let pixel_center: Vec3 =
                pixel_00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction: Vec3 = pixel_center - camera_center;

            let r: Ray = Ray::ray(camera_center, ray_direction);
            let color = color(&r);

            write_color(color);
        }
    }
}
