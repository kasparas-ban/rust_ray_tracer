mod vec3;
use vec3::Vec3;

fn main() {
    /*
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{:} {:}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    
    for j in (0..(IMAGE_HEIGHT-1)).rev() {
        eprintln!("\rScanlines remaining: {:} ", j);
        for i in 0..(IMAGE_WIDTH) {
            let r: f32 = i as f32 / (IMAGE_WIDTH-1) as f32;
            let g: f32 = j as f32 / (IMAGE_HEIGHT-1) as f32;
            let b: f32 = 0.25;

            let ir: u32 = (255.99 * r) as u32;
            let ig: u32 = (255.99 * g) as u32;
            let ib: u32 = (255.99 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nDone.\n");

    */

    let vec1: Vec3 = Vec3::new(1f32, 2f32, 3f32);
    let vec2: Vec3 = Vec3::new(3f32, 2f32, 1f32);
    let vec3: Vec3 = vec1 + vec2;

    println!("\nAdding vectors: {}\n", vec3);

    println!("\nTesting elements: {}", -vec3);
    

}