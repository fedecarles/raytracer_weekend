use crate::{hittable::*, ray::Ray, vec3::Vec3};

#[derive(Debug, Default, Clone, Copy)]
pub struct Material {}

trait Scatter {
    fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> bool;
}
