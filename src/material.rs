use crate::{hittable::*, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3 },
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian {
            albedo: Vec3::default(),
        }
    }
}
//pub trait Material {
//    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> bool;
//}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &Vec3,
        scattered: &Ray,
    ) -> bool {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

                //Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                let scattered = Ray::ray(rec.p, scatter_direction);
                let attenuation = albedo;
                return true;
            }
            Self::Metal { albedo } => {
                let reflected: Vec3 =
                    Vec3::reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
                let scattered = Ray::ray(rec.p, reflected);
                let attenuation = albedo;
                return true;
            }
        }
    }
}
