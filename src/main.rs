use bevy::{
    input::common_conditions::input_just_pressed, math::vec3, prelude::*,
    render::camera::ScalingMode,
};
use lunar_lander::{inputs::*, spacecraft::*};

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

fn update(
    mut windows: Query<&Window>,
    mut players: Query<&mut Player>,
    mut transforms: Query<&mut Transform, With<Player>>,
) {
    let window = windows.single_mut();
    let mut transform = transforms.single_mut();
    let mut player = players.single_mut();
    player.check_boundary(window.resolution.physical_size().as_vec2());
    player.update();
    transform.translation = vec3(player.position.x, player.position.y, player.position.z);
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
            ),
        )
        .run();
}
