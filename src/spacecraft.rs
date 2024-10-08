use crate::vectors::V2;
use bevy::prelude::*;
use libnoise::prelude::*;

/// This will be used to identify the main player entity
#[derive(Component)]
pub struct Player {
    pub position: V2,
    pub velocity: V2,
    pub acceleration: V2,
    noise_x: Billow<1, Simplex<1>>,
    noise_y: Billow<1, Simplex<1>>,
    top_speed: f64,
}

impl Player {
    pub fn origin() -> Player {
        Player {
            position: V2::zeros(),
            velocity: V2::zeros(),
            acceleration: V2::zeros(),
            noise_x: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
            noise_y: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
            top_speed: 5.0,
        }
    }
    pub fn new(position: V2) -> Player {
        Player {
            position,
            velocity: V2::zeros(),
            acceleration: V2::zeros(),
            noise_x: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
            noise_y: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
            top_speed: 5.0,
        }
    }

    pub fn update_randomly(&mut self) {
        let noise_x = self.noise_x.sample([self.position.x as f64]);
        let noise_y = self.noise_y.sample([self.position.y as f64]);

        self.update(&V2::new(noise_x, noise_y));
    }    

    pub fn update(&mut self, new_pos: &V2) {

        let mut delta = V2::sub(new_pos, &self.position);
        delta.self_normalize().self_mul(0.2);
        self.acceleration.set(&delta);
        self.velocity.self_add(&self.acceleration);
        self.position.self_add(&self.velocity);
    }

    pub fn check_boundary(&mut self, boundary: &V2) {
        let boundary_x = boundary.x / 2.0;
        let boundary_y = boundary.y / 2.0;
        if self.position.x >= boundary_x || self.position.x <= -boundary_x {
            self.velocity.x *= -1.0;
        }
        if self.position.y >= boundary_y || self.position.y <= -boundary_y {
            self.velocity.y *= -1.0;
        }
    }
}

#[derive(Component)]
pub struct Ufo;

#[derive(Component)]
pub struct SpacecraftName(pub String);

#[derive(Bundle)]
pub struct Spacecraft {
    name: SpacecraftName,
    sprite: SpriteBundle,
}

impl Spacecraft {
    pub fn new(name: String, sprite: SpriteBundle) -> Self {
        Self {
            name: SpacecraftName(name),
            sprite,
        }
    }
}
