use crate::spacecraft::Player;
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn quit_game(mut exit: EventWriter<AppExit>) {
    info!("Quitting game ...");
    exit.send(AppExit::Success);
}

pub fn keyboard_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let _shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let _ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let space = input.pressed(KeyCode::Space);
    let arrow_left = input.pressed(KeyCode::ArrowLeft);
    let arrow_right = input.pressed(KeyCode::ArrowRight);
    let arrow_up = input.pressed(KeyCode::ArrowUp);
    let arrow_down = input.pressed(KeyCode::ArrowDown);

    let mut transform = query.single_mut();

    if arrow_up {
        move_player_up(&mut transform)
    }
    if arrow_down {
        move_player_down(&mut transform)
    }
    if !space && arrow_left {
        move_player_left(&mut transform)
    }
    if !space && arrow_right {
        move_player_right(&mut transform)
    }
    if space && arrow_left {
        rotate_player_left(&mut transform)
    }
    if space && arrow_right {
        rotate_player_right(&mut transform)
    }
}

fn move_player_up(transform: &mut Transform) {
    transform.translation += Vec3::new(0.0, 1.0, 0.0);
}

fn move_player_down(transform: &mut Transform) {
    transform.translation += Vec3::new(0.0, -1.0, 0.0);
}

fn move_player_left(transform: &mut Transform) {
    transform.translation += Vec3::new(-1.0, 0.0, 0.0);
}

fn move_player_right(transform: &mut Transform) {
    transform.translation += Vec3::new(1.0, 0.0, 0.0);
}

fn rotate_player_left(transform: &mut Transform) {
    transform.rotate_z(0.01);
}

fn rotate_player_right(transform: &mut Transform) {
    transform.rotate_z(-0.01);
}
