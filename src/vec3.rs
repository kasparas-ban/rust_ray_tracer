use crate::rtweekend::*;
use std::fmt;
use std::ops;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn unit_vec(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0],
            ],
        }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            e: [random_f32(), random_f32(), random_f32()],
        }
    }

    pub fn random_limits(min: f32, max: f32) -> Vec3 {
        Vec3 {
            e: [
                random_limits(min, max),
                random_limits(min, max),
                random_limits(min, max),
            ],
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p: Vec3 = Vec3::random_limits(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_limits(-1.0, 1.0), random_limits(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    let a: f32 = random_limits(0.0, 2.0 * PI);
    let z: f32 = random_limits(-1.0, 1.0);
    let r: f32 = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere: Vec3 = random_in_unit_sphere();
    if Vec3::dot(&in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * (*n) * Vec3::dot(&v, &n)
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = Vec3::dot(&-(*uv), n);
    let r_out_perp = etai_over_etat * (*uv + cos_theta*(*n));
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * (*n);
    r_out_perp + r_out_parallel
}

// ================

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let k = 1.0 / rhs;

        Vec3 {
            e: [self.e[0] * k, self.e[1] * k, self.e[2] * k],
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {}, {} }}", self.e[0], self.e[1], self.e[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, -2.0, -5.0),
            Vec3::new(2.0, 0.0, -2.0)
        );
    }

    #[test]
    fn test_vec3_sub() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) - Vec3::new(3.0, 1.0, 3.0),
            Vec3::new(-2.0, 1.0, 0.0)
        );
    }

    #[test]
    fn test_vec3_mul() {
        assert_eq!(
            Vec3::new(0.0, 2.0, 3.0) * Vec3::new(6.0, 3.0, -2.0),
            Vec3::new(0.0, 6.0, -6.0)
        );
    }

    #[test]
    fn test_vec3_scalar_mul() {
        assert_eq!(Vec3::new(0.0, 2.0, -3.0) * 2.0, Vec3::new(0.0, 4.0, -6.0));
    }

    #[test]
    fn test_vec3_div() {
        assert_eq!(Vec3::new(0.0, 8.0, -4.0) / 2.0, Vec3::new(0.0, 4.0, -2.0));
    }

    #[test]
    fn test_vec3_negate() {
        assert_eq!(-Vec3::new(0.0, 8.0, -4.0), Vec3::new(0.0, -8.0, 4.0));
    }
}
