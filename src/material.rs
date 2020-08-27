use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::*;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Color::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

impl Scatter for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_direction = rec.normal + random_unit_vector();
                *scattered = Ray::new(rec.p, scatter_direction);
                *attenuation = albedo.clone();
                true
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(&Vec3::unit_vec(&r_in.direction()), &rec.normal);
                *scattered = Ray::new(rec.p, reflected + (*fuzz)*random_in_unit_sphere());
                *attenuation = albedo.clone();
                Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
            }
        }
    }
}