use bevy::prelude::*;
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

    pub fn self_abs(&self) -> f64 {
        (self.x.powf(2_f64) + self.y.powf(2_f64)).sqrt()
    }

    pub fn self_dot(&mut self, v: &V2) -> f64 {
        (self.x * v.x) + (self.y * v.y)
    }

    pub fn self_add(&mut self, v: &V2) -> &mut Self {
        self.x += v.x;
        self.y += v.y;
        self
    }

    pub fn self_sub(&mut self, v: &V2) -> &mut Self {
        self.x -= v.x;
        self.y -= v.y;
        self
    }

    pub fn sub(v1: &V2, v2: &V2) -> V2 {
        V2::new(v1.x - v2.x, v1.y - v2.y)
    }

    pub fn self_div(&mut self, value: f64) -> &mut Self {
        self.self_mul(1_f64 / value)
    }

    pub fn self_mul(&mut self, value: f64) -> &mut Self {
        self.x *= value;
        self.y *= value;
        self
    }

    pub fn self_normalize(&mut self) -> &mut Self {
        self.self_div(self.self_abs())
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

    pub fn self_limit(&mut self, max: f64) -> &mut Self {
        if self.self_abs() > max {
            self.self_normalize().self_mul(max);
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
        assert_eq!(vector.self_abs(), 5.0);
    }

    #[test]
    fn v2_normalize() {
        let mut vector = V2::new(4.0, 3.0);
        assert_eq!(vector.self_normalize().sefl_abs(), 1.0);
    }
}
