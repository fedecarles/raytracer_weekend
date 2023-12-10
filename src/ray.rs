use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn ray(a: Vec3, b: Vec3) -> Ray {
        Ray { orig: a, dir: b }
    }

    pub fn origin(self) -> Vec3 {
        self.orig
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ray_origin() {}
    #[test]
    fn test_ray_direction() {}
    #[test]
    fn test_ray_point_at_parameter() {}
}
