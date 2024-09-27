use bevy::prelude::*;
use libnoise::prelude::*;

/// This will be used to identify the main player entity
#[derive(Component)]
pub struct Player {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    noise_x: Billow<1, Simplex<1>>,
    noise_y: Billow<1, Simplex<1>>,
}

fn f64_to_f32(x: f64) -> f32{
    let y = x as f32;
    assert_eq!(
        x.is_finite(),
        y.is_finite(),
    );
    y
}

impl Player {
    pub fn origin() -> Player {
        Player {
            position: Vec3::ZERO,
            velocity: Vec3::new(1.0, 1.0, 0.0),
            acceleration: Vec3::ZERO,
            noise_x: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
            noise_y: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
        }
    }
    pub fn new(position: Vec3) -> Player {
        Player {
            position,
            velocity: Vec3::new(1.0, 1.0, 0.0),
            acceleration: Vec3::ZERO,
            noise_x: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
            noise_y: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
        }
    }

    pub fn update(&mut self) {
        //self.velocity += self.acceleration;
        self.position += self.velocity;
        let noise_x = f64_to_f32(self.noise_x.sample([self.position.x as f64]));
        let noise_y = f64_to_f32(self.noise_y.sample([self.position.y as f64]));
        self.velocity += Vec3::new(noise_x, noise_y, 0_f32);
        //info!("Pos {}", self.position);
        //info!("Acc {}", self.acceleration);
        info!("Vel {} Pos {}", self.velocity, self.position);
        
    }

    pub fn check_boundary(&mut self, boundary: Vec2) {
        let boundary_x = boundary.x / 2.0;
        let boundary_y = boundary.y / 2.0;
        if self.position[0] >= boundary_x || self.position[0] <= -boundary_x {
            self.velocity[0] *= -1.0;
        }
        if self.position[1] >= boundary_y || self.position[1] <= -boundary_y {
            self.velocity[1] *= -1.0;
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
