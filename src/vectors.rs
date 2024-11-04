use bevy::prelude::*;
use rand::prelude::*;
use std::{fmt, ops};

pub fn f64_to_f32(x: f64) -> f32 {
    let y = x as f32;
    assert_eq!(x.is_finite(), y.is_finite(),);
    y
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct V2 {
    pub x: f64,
    pub y: f64,
}

impl V2 {
    pub fn new(x: f64, y: f64) -> V2 {
        V2 { x, y }
    }

    pub fn from_vec3(v: &Vec3) -> V2 {
        V2 {
            x: v.x as f64,
            y: v.y as f64,
        }
    }

    pub fn from_vec2(v: &Vec2) -> V2 {
        V2 {
            x: v.x as f64,
            y: v.y as f64,
        }
    }

    pub fn zeros() -> V2 {
        V2 { x: 0.0, y: 0.0 }
    }

    pub fn normalized() -> V2 {
        let mut v = V2::ones();
        v.normalize();
        v
    }

    pub fn ones() -> V2 {
        V2 { x: 1.0, y: 1.0 }
    }

    pub fn x() -> V2 {
        V2 { x: 1.0, y: 0.0 }
    }

    pub fn y() -> V2 {
        V2 { x: 0.0, y: 1.0 }
    }

    pub fn random() -> V2 {
        V2 {
            x: rand::thread_rng().gen(),
            y: rand::thread_rng().gen(),
        }
    }

    pub fn mag(&self) -> f64 {
        self.pow2_mag().sqrt()
    }

    pub fn pow2_mag(&self) -> f64 {
        self.x.powf(2_f64) + self.y.powf(2_f64)
    }

    pub fn dot(&mut self, v: &V2) -> f64 {
        (self.x * v.x) + (self.y * v.y)
    }

    pub fn add(&mut self, v: &V2) -> &mut Self {
        self.x += v.x;
        self.y += v.y;
        self
    }

    pub fn sub(&mut self, v: &V2) -> &mut Self {
        self.x -= v.x;
        self.y -= v.y;
        self
    }

    pub fn sub_vectors(v1: &V2, v2: &V2) -> V2 {
        V2::new(v2.x - v1.x, v2.y - v1.y)
    }

    pub fn div(&mut self, value: f64) -> &mut Self {
        self.mult(1_f64 / value)
    }

    pub fn mult(&mut self, value: f64) -> &mut Self {
        self.x *= value;
        self.y *= value;
        self
    }

    pub fn normalize(&mut self) -> &mut Self {
        self.div(self.mag())
    }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(f64_to_f32(self.x), f64_to_f32(self.y), 0.0)
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(f64_to_f32(self.x), f64_to_f32(self.y))
    }

    pub fn set(&mut self, v: &V2) -> &mut Self {
        self.x = v.x;
        self.y = v.y;
        self
    }

    pub fn limit(&mut self, max: f64) -> &mut Self {
        if self.mag() > max {
            self.normalize().mult(max);
        }
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.x = 0.0;
        self.y = 0.0;
        self
    }

    pub fn set_mag(&mut self, mag: f64) -> &mut Self {
        self.normalize().mult(mag);
        self
    }
}

impl fmt::Display for V2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

impl ops::Add for V2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for V2 {
    fn add_assign(&mut self, rhs: V2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for V2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for V2 {
    fn sub_assign(&mut self, rhs: V2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn v2_abs() {
        let vector = V2::new(4.0, 3.0);
        assert_eq!(vector.mag(), 5.0);
    }

    #[test]
    fn v2_normalize() {
        let mut vector = V2::new(4.0, 3.0);
        assert_eq!(vector.normalize().mag(), 1.0);
    }
}
