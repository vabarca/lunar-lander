use bevy::prelude::*;
use bevy::input::common_conditions::input_just_pressed;
use lunar_lander::{
    spacecraft::*,
    inputs::*,
};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Spacecraft::new("apollo".to_string(),
                                         SpriteBundle {
                                                texture: asset_server.load("sprite/space/spacecraft_128x128.png"),
                                                transform: Transform::from_xyz(25.0, 50.0, 0.0).with_scale(Vec3::splat(0.5)),
                                                ..Default::default()}));
}

fn _update(query: Query<&SpacecraftName, With<Player>>) {
    for name in &query {
        println!("hola {}", name.0);
    }
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        //.add_systems(Update, _update)
        .add_systems(Update, 
            (
                quit_game.run_if(input_just_pressed(KeyCode::KeyQ)),
                move_player_up.run_if(input_just_pressed(KeyCode::ArrowUp)),
                move_player_down.run_if(input_just_pressed(KeyCode::ArrowDown)),
                move_player_left.run_if(input_just_pressed(KeyCode::ArrowLeft)),
                move_player_right.run_if(input_just_pressed(KeyCode::ArrowRight)),
            ))
        .run();
}
