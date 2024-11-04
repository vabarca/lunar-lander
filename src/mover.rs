use crate::forces::{Force, Friction};
use crate::utils::random_coordinates;
use crate::vectors::V2;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
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
    pub fn origin(radius: f64) -> Mover {
        Mover::new(V2::zeros(), radius)
    }
    pub fn new(pos: V2, radius: f64) -> Mover {
        Mover {
            pos,
            vel: V2::zeros(),
            forces: Force::zero(),
            mass: radius * radius * 3.141516,
        }
    }

    pub fn new_random(rect: &Rect, radius: f64) -> Mover {
        Mover::new(random_coordinates(rect), radius)
    }

    pub fn apply_force(&mut self, force: Force) {
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

    pub fn show(&self, tranform: &mut Transform) {
        tranform.translation = self.pos.as_vec3();
    }

    fn surface_contact(&self) -> bool {
        self.pos.y <= 500.0
    }

    fn ground_contact(&self) -> bool {
        self.pos.y <= self.mass
    }

    pub fn check_surface(&mut self, c : f64) {
        if self.surface_contact() {
            let friction = Friction::new(&self.vel, c);
            self.apply_force(friction.f);
        }
    }

    pub fn check_boundary(&mut self) {
        const BOUNCE_LOST: f64 = -0.85;

        if self.ground_contact() {
            self.vel.y *= BOUNCE_LOST;
            self.pos.y = self.mass
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ufo;

pub fn spawn_ufos(
    cmd: &mut Commands, 
    rect: &Rect,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>
) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let radius: f64 = rng.gen::<f64>() * 10f64 + 1.0;
        let pos = random_coordinates(rect);
        info!("Ufo: {} - {}", pos, radius);
        let color = Color::hsl(250.0, 0.95, 0.7);
        cmd.spawn((
            Ufo, 
            Mover::new(pos.clone(), radius),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(radius as f32))),
                material: materials.add(color),
                transform: Transform::from_translation(pos.as_vec2().extend(1.0)),
                ..default()
            },
        ));
    }
}

pub fn spawn_player(
    cmd: &mut Commands,
    rect: &Rect,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let radius: f64 = 40f64;
    let pos = random_coordinates(rect);
    info!("Player: {} - {}", pos, radius);
    cmd.spawn((
        Player,
        Mover::new(pos.clone(), radius),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(radius as f32))),
            material: materials.add(Color::hsl(100.0, 0.2, 0.7)),
            transform: Transform::from_translation(pos.as_vec2().extend(-1.0)),
            ..default()
        },
    ));
}
