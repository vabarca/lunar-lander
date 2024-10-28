use bevy::{input::common_conditions::input_just_pressed, prelude::*, render::camera::ScalingMode};
use bevy_vector_shapes::prelude::*;
use lunar_lander::{cameras::*, forces::*, inputs::*, mover::*, vectors::*};

fn setup(mut commands: Commands, mut windows: Query<&Window>) {
    let mut custom_camera = Camera2dBundle::default();

    custom_camera.projection.scaling_mode = ScalingMode::FixedVertical(1024.0);
    custom_camera.transform = Transform::from_xyz(0.0, 0.0, 0.0);

    let window = windows.single_mut();
    let screen = window.resolution.physical_size().as_vec2();
    let pos1 = V2::new((screen.x / 4.0) as f64, 0.0);
    let pos2 = V2::new((-screen.x / 4.0) as f64, 0.0);

    commands.spawn((custom_camera, CustomCamera));
    commands.spawn((Player, Mover::new(pos1.clone(), 100_f64)));
    commands.spawn((Ufo, Mover::new(pos2.clone(), 50_f64)));
}

fn update(
    windows: Query<&Window>,
    mut query: Query<&mut Mover, With<Player>>,
    mut painter: ShapePainter,
) {
    let mut mover = query.single_mut();
    let mass = mover.mass();
    let window = &windows.single();

    mover.apply_force(&Force::gravity(mass, 0.05));
    if mover.contact(&window) {
        let friction = Friction::new(&mover.vel, 0.1);
         mover.apply_force(&friction.f);
    }
    mover.check_boundary(&window);

    mover.update();
    mover.show(&mut painter);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(Shape2dPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                quit_game.run_if(input_just_pressed(KeyCode::KeyQ)),
                keyboard_input_system,
                mouse_input_system,
                cursor_position,
                update,
            ),
        )
        .run();
}

