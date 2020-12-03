use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z + rhs.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z + self.z
    }

    pub fn normalized(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::from(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
