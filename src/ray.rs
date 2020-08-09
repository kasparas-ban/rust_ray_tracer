use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_at() {

        let ray: Ray = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 0.0, 0.0));

        assert_eq!(
            ray.at(0.0),
            Vec3::new(1.0, 2.0, 3.0)
        );

        assert_eq!(
            ray.at(1.0),
            Vec3::new(2.0, 2.0, 3.0)
        );
    }
}