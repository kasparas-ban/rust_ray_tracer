// Constants

pub const infinity: f32 =  std::f32::MAX;
pub const pi: f32 = 3.1415926535897932385;

// Utility Functions

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * pi / 180.0
}