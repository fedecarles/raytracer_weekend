use crate::utils::reflectance;
use rand::random;
use std::ops::Neg;

use crate::{hittable::*, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ir: f32 },
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian {
            albedo: Vec3::default(),
        }
    }
}

pub fn scatter(
    material: &Material,
    r_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
            *scattered = Ray::ray(rec.p, target - rec.p);
            *attenuation = albedo;
            return true;
        }
        &Material::Metal { albedo, fuzz } => {
            let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
            *scattered = Ray::ray(rec.p, reflected + fuzz * Vec3::random_unit_vector());
            *attenuation = albedo;
            return Vec3::dot(&scattered.direction(), &rec.normal) > 0.0;
        }
        &Material::Dielectric { ir } => {
            *attenuation = Vec3::new(1.0, 1.0, 1.0);
            let refraction_ratio = if rec.front_face { ir.recip() } else { ir };

            let unit_direction = Vec3::unit_vector(r_in.direction());
            let cos_theta = Vec3::dot(&unit_direction.neg(), &rec.normal);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

            let cannot_refract = refraction_ratio * sin_theta > 1.0;

            let direction = if cannot_refract || reflectance(cos_theta, ir) > random::<f32>() {
                Vec3::reflect(unit_direction, rec.normal)
            } else {
                Vec3::refract(unit_direction, rec.normal, refraction_ratio)
            };

            *scattered = Ray::ray(rec.p, direction);
            return true;
        }
    }
}
