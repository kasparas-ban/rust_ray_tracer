use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::*;
use crate::rtweekend::random_f32;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { ref_idx: f32 },
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
            Material::Dielectric { ref_idx } => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                let mut etai_over_etat: f32 = *ref_idx;
                if rec.front_face { etai_over_etat = 1.0 / ref_idx; }

                let unit_direction = Vec3::unit_vec(&r_in.direction());

                let cos_theta = Vec3::dot(&-unit_direction, &rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
                if etai_over_etat * sin_theta > 1.0 {
                    let reflected = reflect(&unit_direction, &rec.normal);
                    *scattered = Ray::new(rec.p, reflected);
                    return true;
                }

                let reflect_prob = schlick(cos_theta, etai_over_etat);
                if random_f32() < reflect_prob {
                    let reflected = reflect(&unit_direction, &rec.normal);
                    *scattered = Ray::new(rec.p, reflected);
                    return true;
                }

                let refracted = refract(&unit_direction, &rec.normal, etai_over_etat);
                *scattered = Ray::new(rec.p, refracted);
                true
            }
        }
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}