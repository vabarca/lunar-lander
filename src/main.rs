use bevy::prelude::*;
use bevy::input::common_conditions::input_just_pressed;
use lunar_lander::{
    spacecraft::*,
    inputs::*,
};

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Spacecraft::new("apollo".to_string()));
}

fn update(query: Query<&SpacecraftName, With<Player>>) {
    for name in &query {
        println!("hola {}", name.0);
    }
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_systems(Update, 
            (
                quit_game.run_if(input_just_pressed(KeyCode::KeyQ)),
            ))
        .run();
}
