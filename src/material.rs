use crate::{hittable::*, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
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
    }
}
