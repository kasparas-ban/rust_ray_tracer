mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use rtweekend::*;
use sphere::*;
use vec3::{Color, Point3, Vec3};

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(r, 0.0, infinity, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
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

    // World

    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let cam: Camera = Camera::new();

    // Render

    println!("P3\n{:} {:}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..(IMAGE_HEIGHT - 1)).rev() {
        eprintln!("\rScanlines remaining: {:} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u: f32 = (i as f32 + random()) / (IMAGE_WIDTH - 1) as f32;
                let v: f32 = (j as f32 + random()) / (IMAGE_HEIGHT - 1) as f32;
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            color::write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.\n");
}
