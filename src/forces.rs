use crate::vectors::V2;

#[derive(Debug, Clone)]
pub struct Force {
    pub v: V2,
}

impl Force {
    pub fn zero() -> Force {
        Force { v: V2::zeros() }
    }
    pub fn new(force: &V2) -> Force {
        Force { v: force.clone() }
    }

    pub fn gravity(mass: f64, scale: f64) -> Force {
        Force::new(V2::new(0.0, -9.8 * scale).mult(mass))
    }

    pub fn reset(&mut self) {
        self.v.reset();
    }
}