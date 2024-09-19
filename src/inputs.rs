use bevy::{ecs::query, prelude::*};
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

pub fn move_player_up(query_name: Query<&SpacecraftName, With<Player>>){
    info_move_player_dir(&query_name, "UP");
}

pub fn move_player_down(query_name: Query<&SpacecraftName, With<Player>>){
    info_move_player_dir(&query_name, "DOWN");
}

pub fn move_player_left(query_name: Query<&SpacecraftName, With<Player>>,
                        mut query_sprite: Query<&Sprite, With<Player>>){
    info_move_player_dir(&query_name, "LEFT");
    for sprite in &query_sprite{
        sprite.tr
    }
}

pub fn move_player_right(query_name: Query<&SpacecraftName, With<Player>>){
    info_move_player_dir(&query_name, "RIGHT");
}


