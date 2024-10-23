use crate::vectors::V2;
use bevy::prelude::*;

#[derive(Debug, Component, Clone)]
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

/// This will be used to identify the main player entity
#[derive(Component, Clone)]
pub struct Mover {
    pub pos: V2,
    pub vel: V2,
    pub forces: Force,
    mass: f64,
}

impl Mover {
    pub fn origin(mass: f64) -> Mover {
        Mover::new(mass, V2::zeros())
    }
    pub fn new(mass: f64, pos: V2) -> Mover {
        Mover {
            pos,
            vel: V2::zeros(),
            forces: Force::zero(),
            mass,
        }
    }

    pub fn apply_force(&mut self, force: &Force) {
        self.forces.v.add(&force.v);
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn update(&mut self) {
        self.forces.v.div(self.mass);
        info!("f {:?}", self.forces.v);
        self.vel.add(&self.forces.v);
        self.pos.add(&self.vel);

        self.forces.reset();
    }

    pub fn check_boundary(&mut self, boundary: &V2) {
        let boundary_x = boundary.x / 2.0;
        let boundary_y = boundary.y / 2.0;
        if self.pos.x >= boundary_x{
            self.pos.x -= boundary.x;
        } else if self.pos.x <= -boundary_x {
            self.pos.x += boundary.x;
        }

        if self.pos.y <= -boundary_y {
            self.vel.mult(-1.0);
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ufo;

#[derive(Component)]
pub struct SpacecraftName(pub String);

#[derive(Bundle)]
pub struct Spacecraft {
    pub name: SpacecraftName,
    pub sprite: SpriteBundle,
}

impl Spacecraft {
    pub fn new(name: String, sprite: SpriteBundle) -> Self {
        Self {
            name: SpacecraftName(name),
            sprite,
        }
    }
}
