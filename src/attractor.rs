use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use crate::{forces::Force, mover::Mover, utils::random_coordinates, vectors::V2};


/// This will be used to identify the main player entity
#[derive(Component, Clone)]
pub struct Attractor {
    pub pos: V2,
    mass: f64,
}

impl Attractor {
    pub fn origin(mass: f64) -> Attractor {
        Attractor::new(V2::zeros(), mass)
    }

    pub fn new(pos: V2, mass: f64) -> Attractor {
        Attractor {
            pos,
            mass,
        }
    }

    pub fn new_random(rect: &Rect, mass: f64) -> Attractor {
        Attractor::new(random_coordinates(rect), mass)
    }

    pub fn new_midle(rect: &Rect, mass: f64) -> Attractor {
        let diff = rect.max - rect.min;
        Attractor::new(V2::new(diff.x as f64/2.0, diff.y as f64/2.0), mass)
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn attract(&self, mover : &Mover) -> Force {
        const G :f64 = 1.0;
        let mut force = self.pos - mover.pos;
        let pow2_distance = force.pow2_mag();
        let mag =  G * self.mass() * mover.mass() / pow2_distance;
        info!("{}", mag);
        force.set_mag(mag);
        Force::new(&force)
    }
}

pub fn spawn_random_attractor(
    cmd: &mut Commands,
    rect: &Rect,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mass: f64 = 80f64;
    let pos = random_coordinates(rect);
    cmd.spawn((
        Attractor::new(pos.clone(), mass),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(mass as f32))),
            material: materials.add(Color::hsl(100.0, 0.95, 0.7)),
            transform: Transform::from_translation(pos.as_vec2().extend(-1.0)),
            ..default()
        },
    ));
}

pub fn spawn_attractor(
    cmd: &mut Commands,
    rect: &Rect,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mass: f64 = 80f64;
    let attractor = Attractor::new_midle(rect, mass);
    let pos = attractor.pos.clone();
    cmd.spawn((
        attractor,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(mass as f32))),
            material: materials.add(Color::hsl(100.0, 0.95, 0.7)),
            transform: Transform::from_translation(pos.as_vec2().extend(-1.0)),
            ..default()
        },
    ));
}
