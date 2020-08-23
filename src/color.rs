use crate::rtweekend::clamp;
use crate::vec3::Color;

//type Color = Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let mut r: f32 = pixel_color.x();
    let mut g: f32 = pixel_color.y();
    let mut b: f32 = pixel_color.z();

    // Divide the color by the number of samples
    let scale: f32 = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    println!(
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as u32,
        (256.0 * clamp(g, 0.0, 0.999)) as u32,
        (256.0 * clamp(b, 0.0, 0.999)) as u32
    );
}
