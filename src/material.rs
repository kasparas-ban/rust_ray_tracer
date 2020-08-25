use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Color::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool {
        false
    }
}

impl Scatter for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_direction = rec.normal + random_unit_vector();
                let scattered = Ray::new(rec.p, scatter_direction);
                let attenuation = albedo;
                true
            }
            Material::Metal { albedo } => {
                let reflected = reflect(&Vec3::unit_vec(&r_in.direction()), &rec.normal);
                let scattered = Ray::new(rec.p, reflected);
                let attenuation = albedo;
                Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
            }
        }
    }
}
