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
    top_speed: f32,
}

fn f64_to_f32(x: f64) -> f32 {
    let y = x as f32;
    assert_eq!(x.is_finite(), y.is_finite(),);
    y
}

fn normalize(mut v: Vec3) -> Vec3 {
    let norm = v.dot(v).sqrt();
    v /= norm;
    v
}

impl Player {
    pub fn origin() -> Player {
        Player {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            noise_x: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
            noise_y: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
            top_speed: 5.0,
        }
    }
    pub fn new(position: Vec3) -> Player {
        Player {
            position,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            noise_x: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
            noise_y: Source::simplex(rand::random::<u64>()).billow(3, 0.013, 2.0, 0.5),
            top_speed: 5.0,
        }
    }

    pub fn limit_velocity(&mut self) {
        if self.velocity.x > self.top_speed {
            self.velocity.x = self.top_speed;
        }

        if self.velocity.y > self.top_speed {
            self.velocity.y = self.top_speed;
        }

        if self.velocity.x < -self.top_speed {
            self.velocity.x = -self.top_speed;
        }
        if self.velocity.y < -self.top_speed {
            self.velocity.y = -self.top_speed;
        }
    }

    pub fn update(&mut self) {
        let noise_x = f64_to_f32(self.noise_x.sample([self.position.x as f64]));
        let noise_y = f64_to_f32(self.noise_y.sample([self.position.y as f64]));
        self.acceleration = normalize(Vec3::new(noise_x, noise_y, 0_f32)) * 0.5;
        self.velocity += self.acceleration;
        //self.limit_velocity();
        self.position += self.velocity;
        info!(
            "Vel {} Pos {} Acc {}",
            self.velocity, self.position, self.acceleration
        );
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
