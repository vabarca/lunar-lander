use bevy::{input::common_conditions::input_just_pressed, prelude::*, render::camera::ScalingMode};
use lunar_lander::{inputs::*, spacecraft::*, vectors::*};
use bevy::color::palettes::basic::WHITE;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1024.0);

    commands.spawn(camera_bundle);
    commands.spawn((
        Player::origin(),
        Spacecraft::new(
            "Apollo 11".to_string(),
            SpriteBundle {
                texture: asset_server.load("sprite/space/spacecraft_128x128.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.5)),
                ..Default::default()
            },
        ),
    ));
}


fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., WHITE);
}

fn update(
    mut windows: Query<&Window>,
    mut players: Query<&mut Player>,
    mut transforms: Query<&mut Transform, With<Player>>,
) {
    let window = windows.single_mut();
    let mut transform = transforms.single_mut();
    let mut player = players.single_mut();
    let screen = window.resolution.physical_size().as_vec2();
    let screen_v2 = V2::new(screen.x as f64, screen.y as f64);
    player.check_boundary(&screen_v2);
    player.update();
    transform.translation.x = f64_to_f32(player.position.x);
    transform.translation.y = f64_to_f32(player.position.y);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        // Set the Fixed Timestep interval to 60 Hz for FixedUpdate
        //.insert_resource(Time::<Fixed>::from_hz(60.0))
        //.add_systems(FixedUpdate, update)
        .add_systems(
            Update,
            (
                quit_game.run_if(input_just_pressed(KeyCode::KeyQ)),
                keyboard_input_system,
                update,
                draw_cursor,
                //cursor_position,
                //cursor_events,
            ),
        )
        .run();
}
