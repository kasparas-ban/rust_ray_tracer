mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::*;
use ray::Ray;
use rtweekend::*;
use sphere::*;
use vec3::{Color, Point3, Vec3};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    let mut rec: HitRecord = HitRecord::default();

    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.mat_ptr.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction: Vec3 = Vec3::unit_vec(&r.direction());
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // World

    let R = (PI/4.0).cos();
    let mut world = HittableList::new();

    let material_ground = Box::new(Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Box::new(Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left = Box::new(Material::Dielectric {
        ref_idx: 1.5,
    });
    let material_right = Box::new(Material::Metal {
        albedo: Color::new(1.0, 0.0, 0.0),
        fuzz: 0.0,
    });

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera

    let cam: Camera = Camera::new(Point3::new(-2.0, 2.0, 1.0),
                                  Point3::new(0.0, 0.0, -1.0),
                                  Vec3::new(0.0, 1.0, 0.0),
                                  20.0, ASPECT_RATIO);

    // Render

    println!("P3\n{:} {:}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..(IMAGE_HEIGHT - 1)).rev() {
        eprintln!("\rScanlines remaining: {:} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u: f32 = (i as f32 + random_f32()) / (IMAGE_WIDTH - 1) as f32;
                let v: f32 = (j as f32 + random_f32()) / (IMAGE_HEIGHT - 1) as f32;
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            color::write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.\n");
}
