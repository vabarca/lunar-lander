use bevy::prelude::*;
use lunar_lander::{
    spacecraft::Spacecraft,
};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    //generate_image();
    commands.spawn(Camera2dBundle::default());

    let sprite = SpriteBundle {
        texture: asset_server.load("sprite/space/spacecraft.png"),
        ..default()
    };
    //commands.spawn((Person, Name("Maria".to_string()), sprite.clone()));
    commands.spawn(Spacecraft::new("apollo".to_string(), 100, 200, sprite));
    //commands.spawn((Person, Name("Antonio".to_string()), sprite.clone()));
}

fn update(mut query: Query<&mut Spacecraft, With<Spacecraft>>) {
    for mut obj in &mut query {
        if obj == "Maria".to_string() {
            break;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}
