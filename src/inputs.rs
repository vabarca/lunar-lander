use bevy::prelude::*;
use bevy::app::AppExit;
use crate::spacecraft::{Player, SpacecraftName};

pub fn quit_game(mut exit: EventWriter<AppExit>){
    info!("Quitting game ...");
    exit.send(AppExit::Success);
}

fn info_move_player_dir(query: &Query<&SpacecraftName, With<Player>>, dir: &str){
    for name in query {
        info!("Move player {} to the {}", name.0, dir);
    }  
}

pub fn move_player_up(query: Query<&SpacecraftName, With<Player>>){
    info_move_player_dir(&query, "UP");
}

pub fn move_player_down(query: Query<&SpacecraftName, With<Player>>){
    info_move_player_dir(&query, "DOWN");
}

pub fn move_player_left(query: Query<&SpacecraftName, With<Player>>){
    info_move_player_dir(&query, "LEFT");
}

pub fn move_player_right(query: Query<&SpacecraftName, With<Player>>){
    info_move_player_dir(&query, "RIGHT");
}


