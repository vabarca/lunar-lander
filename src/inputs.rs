use bevy::prelude::*;
use bevy::app::AppExit;
use crate::spacecraft::Player;

pub fn quit_game(mut exit: EventWriter<AppExit>){
    info!("Quitting game ...");
    exit.send(AppExit::Success);
}

pub fn move_player_up(mut query: Query<&mut Transform, With<Player>>){
    let mut transform = query.single_mut();
    transform.translation += Vec3::new(0.0,1.0,0.0);
}

pub fn move_player_down(mut query: Query<&mut Transform, With<Player>>){
    let mut transform = query.single_mut();
    transform.translation += Vec3::new(0.0,-1.0,0.0);
}

pub fn move_player_left(mut query: Query<&mut Transform, With<Player>>){
    let mut transform = query.single_mut();
    transform.translation += Vec3::new(-1.0,0.0,0.0);
}

pub fn move_player_right(mut query: Query<&mut Transform, With<Player>>){
    let mut transform = query.single_mut();
    transform.translation += Vec3::new(1.0,0.0,0.0);
}


