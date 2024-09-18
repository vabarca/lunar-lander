use bevy::prelude::*;
use bevy::app::AppExit;


pub fn quit_game(mut exit: EventWriter<AppExit>){
    exit.send(AppExit::Success);
}
