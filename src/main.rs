mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
mod rtweekend;

use ray::Ray;
use vec3::{Color, Point3, Vec3};
use sphere::*;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use rtweekend::*;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(r, 0.0, infinity, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction: Vec3 = Vec3::unit_vec(&r.direction());
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    // World

    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Vec3 =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render

    println!("P3\n{:} {:}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..(IMAGE_HEIGHT - 1)).rev() {
        eprintln!("\rScanlines remaining: {:} ", j);
        for i in 0..(IMAGE_WIDTH) {
            let u: f32 = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v: f32 = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let r: Ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color: Color = ray_color(&r, &world);
            color::write_color(pixel_color);
        }
    }

    eprintln!("\nDone.\n");
}
