use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(cen: Point3, r: f32) -> Sphere {
        Sphere { center: cen, radius: r }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = r.direction().length_squared();
        let half_b: f32 = Vec3::dot(&oc, &r.direction());
        let c: f32 = oc.length_squared() - self.radius*self.radius;
        let discriminant: f32 = half_b*half_b - a*c;
    
        if discriminant > 0.0 {
            let root: f32 = discriminant.sqrt();
    
            let temp: f32 = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                return true;
            }
    
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                return true;
            }
        }
    
        return false;
    }
}