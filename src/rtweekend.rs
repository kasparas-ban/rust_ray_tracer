use rand::Rng;

// Constants

pub const INFINITY: f32 = std::f32::MAX;
pub const PI: f32 = 3.1415926535897932385;

// Utility Functions

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_limits(min: f32, max: f32) -> f32 {
    min + (max - min) * random()
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
