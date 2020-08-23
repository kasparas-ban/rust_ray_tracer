use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    aspect_ratio: f32,
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,

    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 16.0 / 9.0,
            viewport_height: 2.0,
            viewport_width: 16.0 / 9.0 * 2.0,
            focal_length: 1.0,

            origin: Point3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(16.0 / 9.0 * 2.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            lower_left_corner: Point3::new(0.0, 0.0, 0.0)
                - Vec3::new(16.0 / 9.0 * 2.0, 0.0, 0.0) / 2.0
                - Vec3::new(0.0, 2.0, 0.0) / 2.0
                - Vec3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
