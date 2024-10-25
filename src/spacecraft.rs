use crate::vectors::V2;
use crate::forces::Force;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

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
        Mover::new(V2::zeros(), mass)
    }
    pub fn new(pos: V2, mass: f64) -> Mover {
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
        self.vel.add(&self.forces.v);
        self.pos.add(&self.vel);
        self.forces.reset();
    }

    pub fn show(&self, painter: &mut ShapePainter) {
        painter.translate(self.pos.as_vec3());
        painter.color = Color::WHITE;
        painter.circle(self.mass as f32);
    }

    pub fn check_boundary(&mut self, window: &Window) {
        let screen = window.resolution.physical_size().as_vec2();
        let boundary = V2::new(screen.x as f64, screen.y as f64);

        let boundary_x = boundary.x / 2.0;
        let boundary_y = boundary.y / 2.0;
        if self.pos.x >= boundary_x {
            self.pos.x -= boundary.x;
        } else if self.pos.x <= -boundary_x {
            self.pos.x += boundary.x;
        }

        if self.pos.y <= -boundary_y {
            self.vel.y *= -1.0;
            self.pos.y = -boundary_y
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ufo;
