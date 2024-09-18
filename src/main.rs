use bevy::prelude::*;
use lunar_lander::spacecraft::Spacecraft;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());

    let sprite = SpriteBundle {
        texture: asset_server.load("sprite/space/spacecraft.png"),
        ..default()
    };
    commands.spawn(Spacecraft::new("apollo".to_string(), 100, 200, sprite));
}

fn update(query: Query<&Spacecraft>) {
    for obj in &query {
        if obj.get_name() == "apollo".to_string() {
            println!("hola {}", obj.get_name());
        }
        else {
            println!("no te conozco");
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
