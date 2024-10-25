use bevy::{input::common_conditions::input_just_pressed, prelude::*, render::camera::ScalingMode};
use bevy_vector_shapes::prelude::*;
use lunar_lander::{inputs::*, spacecraft::*, vectors::*};

fn setup(mut commands: Commands, mut windows: Query<&Window>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1024.0);
    camera_bundle.transform = Transform::from_xyz(100.0, 200.0, 0.0);

    let window = windows.single_mut();
    let screen = window.resolution.physical_size().as_vec2();
    let pos1 = V2::new((screen.x / 4.0) as f64, 0.0);
    let pos2 = V2::new((-screen.x / 4.0) as f64, 0.0);

    commands.spawn(camera_bundle);
    commands.spawn((Player, Mover::new(pos1.clone(), 100_f64)));
    commands.spawn((Ufo, Mover::new(pos2.clone(), 50_f64)));
}

fn update(
    mut windows: Query<&Window>,
    mut query: Query<&mut Mover, With<Player>>,
    mut painter: ShapePainter,
) {
    let mut mover = query.single_mut();
    let mass = mover.mass();
    mover.apply_force(&Force::gravity(mass, 0.05));
    mover.update();
    mover.check_boundary(&windows.single_mut());
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
                update,
            ),
        )
        .run();
}

