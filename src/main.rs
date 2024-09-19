use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use lunar_lander::{inputs::*, spacecraft::*};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player,
        Spacecraft::new(
            "Apollo 11".to_string(),
            SpriteBundle {
                texture: asset_server.load("sprite/space/spacecraft_128x128.png"),
                transform: Transform::from_xyz(25.0, 50.0, 0.0).with_scale(Vec3::splat(0.5)),
                ..Default::default()
            },
        ),
    ));
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
            ),
        )
        .run();
}
