use crate::forces::*;
use crate::utils::*;
use crate::vectors::V2;
use bevy::prelude::*;
use rand::prelude::*;

/// This will be used to identify the main player entity
#[derive(Component, Clone)]
pub struct Body {
    pub name: String,
    pub pos: V2,
    pub vel: V2,
    pub forces: Force,
    pub radius: f64,
    pub mass: f64,
}

impl Body {
    pub fn new(name: String, pos: V2, radius: f64) -> Body {
        Body {
            name,
            pos,
            vel: V2::zeros(),
            forces: Force::zero(),
            radius,
            mass: radius * radius * 3.141516,
        }
    }

    pub fn apply_force(&mut self, force: Force) {
        self.forces.vec.add(&force.vec);
    }

    pub fn update(&mut self) {
        self.forces.vec.div(self.mass);
        self.vel.add(&self.forces.vec);
        self.pos.add(&self.vel);
        self.forces.reset();
    }

    pub fn show(&self, tranform: &mut Transform) {
        tranform.translation = self.pos.as_vec2().extend(tranform.translation.z);
    }

    fn ground_contact(&self) -> bool {
        self.pos.y <= self.mass
    }

    fn body_contact(&mut self, body: &Body) -> bool {
        let diff = body.pos - self.pos;
        diff.mag() <= self.radius + body.radius
    }

    pub fn check_collitions(&mut self, body: &Body) {
        if self.body_contact(body) {
            const BOUNCE_LOST: f64 = -0.9;
            self.vel.mult(BOUNCE_LOST);
            let mut diff = body.pos - self.pos;
            diff.set_mag(self.radius + body.radius);
            self.pos = diff + body.pos;
        }
    }

    pub fn check_boundary(&mut self) {
        const BOUNCE_LOST: f64 = -0.85;

        if self.ground_contact() {
            self.vel.y *= BOUNCE_LOST;
            self.pos.y = self.mass
        }
    }

    fn constrain(distance: f64, min: f64, max: f64) -> f64 {
        if distance < min {
            return min;
        }
        if distance > max {
            return max;
        }
        distance
    }

    pub fn be_attracted(&mut self, body: &Body) {
        const G: f64 = 800.0;
        let mut force = body.pos - self.pos;
        let min_distance = self.mass + body.mass;
        let distance = Body::constrain(force.mag(), min_distance, min_distance * 2.0);
        let mag = G * self.mass * body.mass / (distance * distance);
        force.set_mag(mag);
        self.apply_force(Force::new(&force));
    }

    pub fn friction(&mut self, c: f64) {
        let mut friction = Force::new(&self.vel);
        let pow2_speed = self.vel.pow2_mag();
        friction.vec.mult(-1.0).set_mag(c * pow2_speed);
        self.apply_force(friction);
    }
}

#[derive(Component)]
pub struct Attractor;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ufo;

#[derive(Bundle)]
pub struct AttractorBundle {
    attractor: Attractor,
    body: Body,
}

impl AttractorBundle {
    pub fn new(name: String, pos: V2, radius: f64) -> Self {
        Self {
            body: Body::new(name, pos, radius),
            attractor: Attractor,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    body: Body,
}

impl PlayerBundle {
    pub fn new(name: String, pos: V2, radius: f64) -> Self {
        Self {
            body: Body::new(name, pos, radius),
            player: Player,
        }
    }
}

#[derive(Bundle)]
pub struct UfoBundle {
    ufo: Ufo,
    body: Body,
}

impl UfoBundle {
    pub fn new(name: String,pos: V2, radius: f64) -> Self {
        Self {
            body: Body::new(name, pos, radius),
            ufo: Ufo,
        }
    }
}

pub fn spawn_player(cmd: &mut Commands, rect: &Rect, asset_server: &Res<AssetServer>) {
    let radius: f64 = 40f64;
    let pos = random_coordinates(rect);
    info!("Player: {} - {}", pos, radius);
    cmd.spawn((
        PlayerBundle::new("Player".to_string(), pos.clone(), radius),
        SpriteBundle {
            texture: asset_server.load("sprite/spacecraft_64x64.png"),
            transform: Transform::from_translation(pos.as_vec2().extend(-1.0)),
            ..default()
        },
    ));
}

pub fn spawn_ufos(cmd: &mut Commands, rect: &Rect, asset_server: &Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    for i in 0..4 {
        let radius: f64 = rng.gen::<f64>() * 10f64 + 1.0;
        let pos = random_coordinates(rect);
        info!("Ufo: {} - {}", pos, radius);
        cmd.spawn((
            UfoBundle::new(format!("Ufo {}", i), pos.clone(), radius),
            SpriteBundle {
                texture: asset_server.load("sprite/ufo_64x64.png"),
                transform: Transform::from_translation(pos.as_vec2().extend(-2.0)),
                ..default()
            },
        ));
    }
}

pub fn spawn_attractor(cmd: &mut Commands, rect: &Rect, asset_server: &Res<AssetServer>) {
    let radius: f64 = 80f64;
    let pos = middle_coordinates(rect);
    info!("Attractor: {} - {}", pos, radius);
    cmd.spawn((
        AttractorBundle::new("Attractor".to_string(), pos.clone(), radius),
        SpriteBundle {
            texture: asset_server.load("sprite/planet_256x256.png"),
            transform: Transform::from_translation(pos.as_vec2().extend(-3.0)),
            ..default()
        },
    ));
}
