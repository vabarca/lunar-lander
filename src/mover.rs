use crate::vectors::V2;
use crate::forces::Force;
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

    pub fn contact(&self, window: &Window) -> bool{
        let screen = window.resolution.physical_size().as_vec2();
        let boundary = V2::new(screen.x as f64, screen.y as f64);
        let boundary_y = boundary.y / 2.0;
        //info!("{} - {} - {} - {}", self.pos.y, boundary_y, self.mass, (-boundary_y - self.mass - 1.0));
        self.pos.y < (-boundary_y - self.mass - 2.0)
    }

    pub fn check_boundary(&mut self, window: &Window) {
        let screen = window.resolution.physical_size().as_vec2();
        let boundary = V2::new(screen.x as f64, screen.y as f64);
        const BOUNCE_LOST: f64 = -0.85;

        let boundary_y = boundary.y / 2.0;

        
        if self.pos.y - self.mass <= -boundary_y {
            self.vel.y *= BOUNCE_LOST;
            self.pos.y = -boundary_y + self.mass
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ufo;

pub fn spawn_player(
    cmd: &mut Commands, 
){
    cmd.spawn((Player, Mover::new(V2::new(0.0, 0.0), 40_f64)));
}

pub fn spawn_ufos(
    cmd: &mut Commands, 
    window: &mut Window
){
    let mut rng = rand::thread_rng();
    window.resizable = false;
    let screen = window.resolution.physical_size().as_vec2();
    
    for _ in 1..10 {
        let _x = rng.gen::<f64>();
        let _y = rng.gen::<f64>();
        let x: f64 = _x * screen.x as f64; 
        let y: f64 = _y * screen.y as f64;
        let mass: f64 = rng.gen::<f64>() * 10f64; 
        info!("New ufo x:{}({}) - y:{}({})", x, _x, y, _y);
        cmd.spawn((Ufo, Mover::new(V2::new(x, y), mass)));
    }
}
