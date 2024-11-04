use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use crate::{forces::Force, mover::Mover, utils::random_coordinates, vectors::V2};


/// This will be used to identify the main player entity
#[derive(Component, Clone)]
pub struct Attractor {
    pub pos: V2,
    mass: f64,
}

impl Attractor {
    pub fn origin(radius: f64) -> Attractor {
        Attractor::new(V2::zeros(), radius)
    }

    pub fn new(pos: V2, radius: f64) -> Attractor {
        Attractor {
            pos,
            mass: radius * radius * 3.141516,
        }
    }

    pub fn new_random(rect: &Rect, radius: f64) -> Attractor {
        Attractor::new(random_coordinates(rect), radius)
    }

    pub fn new_midle(rect: &Rect, radius: f64) -> Attractor {
        let diff = rect.max - rect.min;
        Attractor::new(V2::new(diff.x as f64/2.0, diff.y as f64/2.0), radius)
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    fn constrain(distance: f64, min: f64, max :f64) -> f64{
        if distance < min {
            return min
        }
        if distance > max {
            return max
        }
        distance
    }

    pub fn attract(&self, mover : &Mover) -> Force {
        const G :f64 = 800.0;
        let mut force = self.pos - mover.pos;
        let min_distance = self.mass + mover.mass();
        let distance = Attractor::constrain(force.mag(),min_distance, min_distance * 2.0);
        let mag =  G * self.mass() * mover.mass() / (distance * distance);
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
    let radius: f64 = 80f64;
    let pos = random_coordinates(rect);
    info!("Attractor: {} - {}", pos, radius);
    cmd.spawn((
        Attractor::new(pos.clone(), radius),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(radius as f32))),
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
    let radius: f64 = 80f64;
    let attractor = Attractor::new_midle(rect, radius);
    let pos = attractor.pos.clone();
    info!("Attractor: {} - {}", pos, radius);
    cmd.spawn((
        attractor,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(radius as f32))),
            material: materials.add(Color::hsl(100.0, 0.7, 0.7)),
            transform: Transform::from_translation(pos.as_vec2().extend(-1.0)),
            ..default()
        },
    ));
}
