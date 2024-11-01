use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct Corner;

#[derive(Component)]
pub struct CornerTexts;

pub fn spawn_corners(
    cmd: &mut Commands,
    window: &mut Window,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Annulus::new(2.0, 8.0)));
    let position = [
        Transform::from_xyz(0.0, 0.0, 0.0),
        Transform::from_xyz(0.0, window.resolution.height(), 0.0),
        Transform::from_xyz(window.resolution.width(), window.resolution.height(), 0.0),
        Transform::from_xyz(window.resolution.width(), 0.0, 0.0),
    ];

    for i in 0..4 {
        let color = Color::hsl(360. * i as f32 / 4 as f32, 0.95, 0.7);
        cmd.spawn((
            Corner,
            MaterialMesh2dBundle {
                mesh: shape.clone(),
                material: materials.add(color),
                transform: position[i],
                ..default()
            },
        ));
    }
}
