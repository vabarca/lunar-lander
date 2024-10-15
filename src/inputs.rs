use crate::spacecraft::Mover;
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn quit_game(mut exit: EventWriter<AppExit>) {
    info!("Quitting game ...");
    exit.send(AppExit::Success);
}

pub fn keyboard_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Mover>>,
) {
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let _ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let space = input.pressed(KeyCode::Space) || input.just_pressed(KeyCode::Space);
    let arrow_left = input.pressed(KeyCode::ArrowLeft) || input.just_pressed(KeyCode::ArrowLeft);
    let arrow_right = input.pressed(KeyCode::ArrowRight) || input.just_pressed(KeyCode::ArrowRight);
    let arrow_up = input.pressed(KeyCode::ArrowUp) || input.just_pressed(KeyCode::ArrowUp);
    let arrow_down = input.pressed(KeyCode::ArrowDown) || input.just_pressed(KeyCode::ArrowDown);

    let mut transform = query.single_mut();

    if !shift && arrow_up {
        move_up(&mut transform)
    }
    if !shift && arrow_down {
        move_down(&mut transform)
    }
    if !space && arrow_left {
        move_left(&mut transform)
    }
    if !space && arrow_right {
        move_right(&mut transform)
    }
    if space && arrow_left {
        rotate_left(&mut transform)
    }
    if space && arrow_right {
        rotate_right(&mut transform)
    }
    if shift && arrow_up {
        scale_up(&mut transform)
    }
    if shift && arrow_down {
        scale_down(&mut transform)
    }
}

fn move_up(transform: &mut Transform) {
    transform.translation += Vec3::new(0.0, 1.0, 0.0);
}

fn move_down(transform: &mut Transform) {
    transform.translation += Vec3::new(0.0, -1.0, 0.0);
}

fn move_left(transform: &mut Transform) {
    transform.translation += Vec3::new(-1.0, 0.0, 0.0);
}

fn move_right(transform: &mut Transform) {
    transform.translation += Vec3::new(1.0, 0.0, 0.0);
}

fn rotate_left(transform: &mut Transform) {
    transform.rotate_z(0.01);
}

fn rotate_right(transform: &mut Transform) {
    transform.rotate_z(-0.01);
}

fn scale_up(transform: &mut Transform) {
    transform.scale *= Vec3::splat(1.01);
}

fn scale_down(transform: &mut Transform) {
    transform.scale *= Vec3::splat(0.99);
}
