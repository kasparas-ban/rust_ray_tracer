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
}

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
