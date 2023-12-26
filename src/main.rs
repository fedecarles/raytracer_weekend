use camera::Camera;
use hittable_list::*;
use material::Material;
use rand::random;
use ray::Ray;
use sphere::*;
use vec3::Vec3;

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;

fn main() {
    // World
    let mut world = HittableList::default();

    let ground_material = Material::Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let chose_mat: f32 = random();
            let center = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mut _sphere_material: Material = Material::default();

                if chose_mat < 0.8 {
                    //difuse
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    _sphere_material = Material::Lambertian { albedo: albedo };
                    world.add(Box::new(Sphere::new(center, 0.2, _sphere_material)));
                } else if chose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = random::<f32>();
                    _sphere_material = Material::Metal {
                        albedo: albedo,
                        fuzz: fuzz,
                    };
                    world.add(Box::new(Sphere::new(center, 0.2, _sphere_material)));
                } else {
                    // glass
                    _sphere_material = Material::Dielectric { ir: 1.5 };
                    world.add(Box::new(Sphere::new(center, 0.2, _sphere_material)));
                }
            }
        }
    }

    let material1 = Material::Dielectric { ir: 1.5 };
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    };
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Camera
    let mut cam: Camera = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_deph = 50;

    cam.vfov = -20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world)
}
