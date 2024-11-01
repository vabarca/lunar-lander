use crate::forces::Force;
use crate::vectors::V2;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use rand::prelude::*;

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
        self.forces.vec.add(&force.vec);
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn update(&mut self) {
        self.forces.vec.div(self.mass);
        self.vel.add(&self.forces.vec);
        self.pos.add(&self.vel);
        self.forces.reset();
    }

    pub fn show(&self, painter: &mut ShapePainter) {
        painter.translate(self.pos.as_vec3());
        painter.color = Color::WHITE;
        painter.circle(self.mass as f32);
    }

    pub fn contact(&self, window: &Window) -> bool {
        let screen = window.resolution.physical_size().as_vec2();
        let boundary = V2::new(screen.x as f64, screen.y as f64);
        let boundary_y = boundary.y / 2.0;
        //info!("{} - {} - {} - {}", self.pos.y, boundary_y, self.mass, (-boundary_y - self.mass - 1.0));
        self.pos.y < (-boundary_y - self.mass - 2.0)
    }

    pub fn check_boundary(&mut self) {
        const BOUNCE_LOST: f64 = -0.85;

        if self.pos.y - self.mass <= 0.0 {
            self.vel.y *= BOUNCE_LOST;
            self.pos.y = self.mass
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ufo;

pub fn spawn_player(cmd: &mut Commands, rect: &Rect) {
    let mut rng = rand::thread_rng();
    let mass: f64 = 40f64;
    let diff = rect.max - rect.min;
    let x: f64 = rng.gen::<f64>() * diff.x as f64 + rect.min.x as f64;
    let y: f64 = rng.gen::<f64>() * diff.y as f64 + rect.min.y as f64  + mass;


    cmd.spawn((Player, Mover::new(V2::new(x, y), mass)));
}

pub fn spawn_ufos(cmd: &mut Commands, rect: &Rect) {
    let mut rng = rand::thread_rng();
    let diff = rect.max - rect.min;

    for _ in 0..10 {
        let mass: f64 = rng.gen::<f64>() * 10f64;
        let x: f64 = rng.gen::<f64>() * diff.x as f64 + rect.min.x as f64;
        let y: f64 = rng.gen::<f64>() * diff.y as f64 + rect.min.y as f64  + mass;
        info!("New ufo x:{} - y:{}", x, y);
        cmd.spawn((Ufo, Mover::new(V2::new(x, y), mass)));
    }
}
