use bevy::prelude::*;
use bevy::app::AppExit;


pub fn quit_game(mut exit: EventWriter<AppExit>){
    info!("Quitting game ...");
    exit.send(AppExit::Success);
}

pub fn move_player_up(){
    info!("Move player UP");
}

pub fn move_player_down(){
    info!("Move player DOWN");
}

pub fn move_player_left(){
    info!("Move player LEFT");
}

pub fn move_player_right(){
    info!("Move player RIGHT");
}


