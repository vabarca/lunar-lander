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
        V2 { x: 1.0, y: 1.0 }
    }

    pub fn abs(&self) -> f64 {
        (self.x.powf(2_f64) + self.y.powf(2_f64)).sqrt()
    }

    pub fn dot(&mut self, v: &V3) -> f64 {
        (self.x * v.x) + (self.y * v.y)
    }

    pub fn add(&mut self, v: &V3) -> &mut Self {
        self.x += v.x;
        self.y += v.y;
        self
    }

    pub fn sub(&mut self, v: &V3) -> &mut Self {
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

    pub fn as_v3(&self) -> V3 {
        V3::new(self.x, self.y, 0.0)
    }

    pub fn limit(&mut self, max: f64) -> &mut Self {
        if self.abs() > max {
            self.normalize().mul(max);
        }
        self
    }
}

pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl V3 {
    pub fn new(x: f64, y: f64, z: f64) -> V3 {
        V3 { x, y, z }
    }

    pub fn zeros() -> V3 {
        V3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn ones() -> V3 {
        V3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn random() -> V3 {
        V3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn abs(&self) -> f64 {
        (self.x.powf(2_f64) + self.y.powf(2_f64)).sqrt()
    }

    pub fn dot(&mut self, v: &V3) -> f64 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }

    pub fn add(&mut self, v: &V3) -> &mut Self {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self
    }

    pub fn sub(&mut self, v: &V3) -> &mut Self {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
        self
    }

    pub fn div(&mut self, value: f64) -> &mut Self {
        self.mul(1_f64 / value)
    }

    pub fn mul(&mut self, value: f64) -> &mut Self {
        self.x *= value;
        self.y *= value;
        self.z *= value;
        self
    }

    pub fn normalize(&mut self) -> &mut Self {
        self.div(self.abs())
    }

    pub fn as_v2(&self) -> V2 {
        V2::new(self.x, self.y)
    }

    pub fn limit(&mut self, max: f64) -> &mut Self {
        if self.abs() > max {
            self.normalize().mul(max);
        }
        self
    }
}

#[test]
fn v3_abs() {
    let vector = V3::new(4.0, 3.0, 0.0);
    assert_eq!(vector.abs(), 5.0);
}

#[test]
fn v3_normalize() {
    let mut vector = V3::new(4.0, 3.0, 0.0);
    assert_eq!(vector.normalize().abs(), 1.0);
}

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
