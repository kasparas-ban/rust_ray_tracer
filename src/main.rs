mod vec3;
use vec3::{Vec3, Color};

mod color;

fn main() {
    
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{:} {:}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    
    for j in (0..(IMAGE_HEIGHT-1)).rev() {
        eprintln!("\rScanlines remaining: {:} ", j);
        for i in 0..(IMAGE_WIDTH) {
            let pixel_color: Color = Color::new(i as f32 / (IMAGE_WIDTH-1) as f32, j as f32 / (IMAGE_HEIGHT-1) as f32, 0.25);
            color::write_color(pixel_color);
        }
    }

    eprintln!("\nDone.\n");

    

    let vec1: Vec3 = Vec3::new(1f32, 2f32, 3f32);
    let vec2: Vec3 = Vec3::new(3f32, 2f32, 1f32);
    let vec3: Vec3 = vec1 + vec2;

    println!("\nAdding vectors: {}\n", vec3);

    println!("\nTesting elements: {}", -vec3);
    
    let color1: Color = Color::new(1f32, 1f32, 1f32);
}