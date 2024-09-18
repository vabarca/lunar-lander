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
                                                texture: asset_server.load("sprite/space/spacecraft.png"),
                                                transform: Transform::from_xyz(25.0, 50.0, 0.0).with_scale(Vec3::new(0.1,0.1,0.1)),
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
            ))
        .run();
}
