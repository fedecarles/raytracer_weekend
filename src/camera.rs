use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::utils::*;
use crate::vec3::Vec3;
use rand::prelude::*;
use std::ops::Range;

fn clamp(rng: &Range<f32>, val: f32) -> f32 {
    if val < rng.start {
        rng.start
    } else if val > rng.end {
        rng.end
    } else {
        return val;
    }
}

fn linear_to_gama(linear_component: f32) -> f32 {
    return linear_component.sqrt();
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    pub aspect_ratio: f32,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_deph: i32,          // Maximum number of ray bounces into scene
    pub vfov: f32,              // Vertical view angle (field of view)
    pub lookfrom: Vec3,         // Point camera is looking from
    pub lookat: Vec3,           // Point camera is looking at
    pub vup: Vec3,              // Camera-relative "up" direction
    pub defocus_angle: f32,     // Variation angle of rays through each pixel
    pub focus_dist: f32,        // Distance from camera lookfrom point to plane of perfect focus
    image_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,              // Camera frame basis vectors
    v: Vec3,              // Camera frame basis vectors
    w: Vec3,              // Camera frame basis vectors
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Camera {
    pub fn render(&mut self, world: &HittableList) {
        // Initialize camera
        self.initialize();
        // Render
        println!("P3\n{} {}\n{}", self.image_width, self.image_height, 255);
        for j in (0..self.image_height).rev() {
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::default();
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color = pixel_color + color(&r, self.max_deph, &world);
                }

                self.write_color(pixel_color, self.samples_per_pixel);
            }
        }
    }
    pub fn write_color(self, pixel_color: Vec3, samples_per_pixel: i32) {
        let mut r = pixel_color.x();
        let mut g = pixel_color.y();
        let mut b = pixel_color.z();

        // Divide the color by the number of samples.

        let scale = 1.0 / samples_per_pixel as f32;
        r *= scale;
        g *= scale;
        b *= scale;

        // Apply the linear to gamma transform.
        r = linear_to_gama(r);
        g = linear_to_gama(g);
        b = linear_to_gama(b);

        // Write the translated [0, 255] value of each color component.
        let intensity: Range<f32> = Range {
            start: 0.000,
            end: 0.999,
        };
        println!(
            "{} {} {}",
            (255.99 * clamp(&intensity, r)) as i32,
            (255.99 * clamp(&intensity, g)) as i32,
            (255.99 * clamp(&intensity, b)) as i32
        )
    }
    pub fn initialize(&mut self) {
        // image size
        self.image_height = std::cmp::max((self.image_width as f32 / self.aspect_ratio) as i32, 1);
        self.center = self.lookfrom;

        // viewport size
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h * self.focus_dist;
        let viewport_width: f32 =
            viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = Vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = Vec3::unit_vector(Vec3::cross(&self.vup, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: Vec3 = viewport_width * -self.u; // Vector across viewport horizontal edge
        let viewport_v: Vec3 = viewport_height * -self.v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left: Vec3 =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * ((self.defocus_angle / 2.0).to_radians()).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn get_ray(self, i: i32, j: i32) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.
        let pixel_center: Vec3 =
            self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample: Vec3 = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction: Vec3 = pixel_sample - ray_origin;

        Ray::ray(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(self) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        return self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v);
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + rng.gen::<f32>();
        let py = -0.5 + rng.gen::<f32>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
