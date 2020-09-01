use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::rtweekend::degrees_to_radians;

pub struct Camera {
    //aspect_ratio: f32,
    //viewport_height: f32,
    //viewport_width: f32,
    //focal_length: f32,

    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3,
               vfov: f32, aspect_ratio: f32) -> Camera {

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let w = Vec3::unit_vec(&(lookfrom - lookat));
        let u = Vec3::unit_vec(&(Vec3::cross(&vup, &w)));
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical =  viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - 
                                vertical/2.0 - w;

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin,
        )
    }
}
