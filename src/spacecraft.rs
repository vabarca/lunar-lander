use bevy::prelude::*;
use perlin_noise::PerlinNoise;

/// This will be used to identify the main player entity
#[derive(Component)]
pub struct Player {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    perlin: PerlinNoise,
}

impl Player {
    pub fn origin() -> Player {
        Player {
            position: Vec3::ZERO,
            velocity: Vec3::splat(2.0),
            acceleration: Vec3::ZERO,
            perlin: PerlinNoise::new(),
        }
    }
    pub fn new(position: Vec3) -> Player {
        Player {
            position,
            velocity: Vec3::ONE,
            acceleration: Vec3::ZERO,
            perlin: PerlinNoise::new(),
        }
    }

    pub fn update(&mut self) {
        //info!("position pre {} velocity {}", self.position, self.velocity);
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration += Vec2::new(f32::from(self.perlin.get2d(self.position.x)), self.perlin.get2d(self.position.x))

        //info!("position post{}", self.position);
        
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
