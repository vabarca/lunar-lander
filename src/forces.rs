use crate::vectors::V2;

#[derive(Debug, Clone)]
pub struct Force {
    pub vec: V2,
}

impl Force {
    pub fn zero() -> Force {
        Force { vec: V2::zeros() }
    }

    pub fn new(force: &V2) -> Force {
        Force { vec: force.clone() }
    }

    pub fn from_vector_points(x:f64 , y: f64) -> Force {
        Force { vec: V2::new(x, y) }
    }

    pub fn gravity(mass: f64) -> Force {
        const SCALE :f64 = 0.05;
        Force::new(V2::new(0.0, -9.8 * SCALE).mult(mass))
    }

    pub fn reset(&mut self) {
        self.vec.reset();
    }
}

