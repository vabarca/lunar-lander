use bevy::{input::common_conditions::input_just_pressed, prelude::*, render::camera::ScalingMode};
use lunar_lander::{inputs::*, spacecraft::*, vectors::*};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: Query<&Window>,) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1024.0);
    camera_bundle.transform = Transform::from_xyz(100.0, 200.0, 0.0);

    let window = windows.single_mut();
    let screen = window.resolution.physical_size().as_vec2();
    let pos1 = V2::new((screen.x/4.0) as f64 , 0.0);
    let pos2 = V2::new((-screen.x/4.0) as f64 , 0.0);

    commands.spawn(camera_bundle);
    commands.spawn((
        Player,
        Mover::new(100_f64, pos1.clone()),
        Spacecraft::new(
            "Apollo 11".to_string(),
            SpriteBundle {
                texture: asset_server.load("sprite/space/spacecraft_128x128.png"),
                transform: Transform::from_xyz(pos1.x as f32 , pos1.y as f32, 0.0).with_scale(Vec3::splat(0.5)),
                ..Default::default()
            },
        ),
    ));
    commands.spawn((
        Ufo,
        Mover::new(50_f64, pos2.clone()),
        Spacecraft::new(
            "Apollo 13".to_string(),
            SpriteBundle {
                texture: asset_server.load("sprite/space/spacecraft_128x128.png"),
                transform: Transform::from_xyz(pos2.x as f32, pos2.y as f32, 0.0).with_scale(Vec3::splat(0.5)),
                ..Default::default()
            },
        ),
    ));
}

fn update(
    mut windows: Query<&Window>,
    mut query: Query<(&mut Transform, &mut Mover)>,
) {
    let window = windows.single_mut();
    let screen = window.resolution.physical_size().as_vec2();
    let screen_v2 = V2::new(screen.x as f64, screen.y as f64);

    for (mut transform, mut mover) in &mut query {
        let mass = mover.mass();
        mover.apply_force(&Force::gravity(mass, 0.05));
        mover.update();
        mover.check_boundary(&screen_v2);
    
        transform.translation.x = f64_to_f32(mover.pos.x);
        transform.translation.y = f64_to_f32(mover.pos.y);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                quit_game.run_if(input_just_pressed(KeyCode::KeyQ)),
                keyboard_input_system,
                update,
            ),
        )
        .run();
}
