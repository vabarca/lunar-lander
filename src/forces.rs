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

    pub fn gravity(mass: f64, scale: f64) -> Force {
        Force::new(V2::new(0.0, -9.8 * scale).mult(mass))
    }

    pub fn reset(&mut self) {
        self.vec.reset();
    }
}

#[derive(Debug, Clone)]
pub struct Friction {
    pub f: Force,
}

impl Friction {
    pub fn new(v: &V2, c: f64) -> Friction{
        let mut friction = Friction{ 
            f: Force::new(v),
        };
        friction.f.vec.mult(-1.0);
        friction.f.vec.set_mag(c);
        friction
    }
}