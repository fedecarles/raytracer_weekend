use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::{hittable::*, vec3::Vec3};
use std::ops::Range;

#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    image_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        // Render
        println!("P3\n{} {}\n{}", self.image_width, self.image_height, 255);
        for j in (0..self.image_height).rev() {
            for i in 0..self.image_width {
                let pixel_center: Vec3 = self.pixel00_loc
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);
                let ray_direction: Vec3 = pixel_center - self.center;

                let r: Ray = Ray::ray(self.center, ray_direction);
                let color = self.color(r, &world);

                self.write_color(color);
            }
        }
    }
    pub fn write_color(self, pixel_color: Vec3) {
        println!(
            "{} {} {}",
            (255.99 * pixel_color.x()) as i32,
            (255.99 * pixel_color.y()) as i32,
            (255.99 * pixel_color.z()) as i32
        )
    }
    pub fn initialize(&mut self) {
        // image size
        self.aspect_ratio = 16.0 / 9.0;
        self.image_width = 400;
        self.image_height = std::cmp::max((self.image_width as f32 / self.aspect_ratio) as i32, 1);
        self.center = Vec3::new(0.0, 0.0, 0.0);

        // viewport size
        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 =
            viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: Vec3 =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn color(self, r: Ray, world: &HittableList) -> Vec3 {
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
}
