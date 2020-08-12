use crate::vec3::Color;

//type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.99 * pixel_color.x()) as u32,
        (255.99 * pixel_color.y()) as u32,
        (255.99 * pixel_color.z()) as u32
    );
}
