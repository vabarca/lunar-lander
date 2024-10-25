use crate::spacecraft::{Player, Mover};
use crate::forces::Force;
use bevy::app::AppExit;
use bevy::prelude::*;
use crate::vectors::V2;

pub fn quit_game(mut exit: EventWriter<AppExit>) {
    info!("Quitting game ...");
    exit.send(AppExit::Success);
}

pub fn mouse_input_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Mover, With<Player>>,
) {
    let mut mover = query.single_mut();
    for _ in buttons.get_pressed() {
        let wind = Force::new(&V2::new(-1.0, 0.0));
        mover.apply_force(&wind);
    }
}

pub fn keyboard_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Mover, With<Player>>,
) {
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let _ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let space = input.pressed(KeyCode::Space) || input.just_pressed(KeyCode::Space);
    let arrow_left = input.pressed(KeyCode::ArrowLeft) || input.just_pressed(KeyCode::ArrowLeft);
    let arrow_right = input.pressed(KeyCode::ArrowRight) || input.just_pressed(KeyCode::ArrowRight);
    let arrow_up = input.pressed(KeyCode::ArrowUp) || input.just_pressed(KeyCode::ArrowUp);
    let arrow_down = input.pressed(KeyCode::ArrowDown) || input.just_pressed(KeyCode::ArrowDown);

    let mut mover = query.single_mut();

    if !shift && arrow_up {
        move_up(&mut mover)
    }
    if !shift && arrow_down {
        move_down(&mut mover)
    }
    if !space && arrow_left {
        move_left(&mut mover)
    }
    if !space && arrow_right {
        move_right(&mut mover)
    }
}

fn move_up(mover: &mut Mover) {
    mover.pos.add(&V2::new(0.0, 1.0));
}

fn move_down(mover: &mut Mover) {
    mover.pos.add(&V2::new(0.0, -1.0));
}

fn move_left(mover: &mut Mover) {
    mover.pos.add(&V2::new(1.0, 0.0));
}

fn move_right(mover: &mut Mover) {
    mover.pos.add(&V2::new(-1.0, 0.0));
}

