use bevy::{a11y::accesskit::Vec2, math::Vec3};
use rand::prelude::*;

pub fn f64_to_f32(x: f64) -> f32 {
    let y = x as f32;
    assert_eq!(x.is_finite(), y.is_finite(),);
    y
}

pub struct V2 {
    pub x: f64,
    pub y: f64,
}

impl V2 {
    pub fn new(x: f64, y: f64) -> V2 {
        V2 { x, y }
    }

    pub fn zeros() -> V2 {
        V2 { x: 0.0, y: 0.0 }
    }

    pub fn ones() -> V2 {
        V2 { x: 1.0, y: 1.0 }
    }

    pub fn random() -> V2 {
        V2 {
            x: rand::thread_rng().gen(),
            y: rand::thread_rng().gen(),
        }
    }

    pub fn abs(&self) -> f64 {
        (self.x.powf(2_f64) + self.y.powf(2_f64)).sqrt()
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

    pub fn div(&mut self, value: f64) -> &mut Self {
        self.mul(1_f64 / value)
    }

    pub fn mul(&mut self, value: f64) -> &mut Self {
        self.x *= value;
        self.y *= value;
        self
    }

    pub fn normalize(&mut self) -> &mut Self {
        self.div(self.abs())
    }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(f64_to_f32(self.x), f64_to_f32(self.y), 0.0)
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn update(&mut self, v: &Vec2) -> &mut Self {
        self.x = v.x;
        self.y = v.y;
        self
    }

    pub fn limit(&mut self, max: f64) -> &mut Self {
        if self.abs() > max {
            self.normalize().mul(max);
        }
        self
    }
}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn v2_abs() {
        let vector = V2::new(4.0, 3.0);
        assert_eq!(vector.abs(), 5.0);
    }

    #[test]
    fn v2_normalize() {
        let mut vector = V2::new(4.0, 3.0);
        assert_eq!(vector.normalize().abs(), 1.0);
    }
}
