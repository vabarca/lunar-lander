use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_vector_shapes::prelude::*;
use lunar_lander::{cameras::*, forces::*, inputs::*, mover::*, vectors::*};
use rand::prelude::*;

fn setup(mut cmd: Commands, mut windows: Query<&mut Window>) {

    let mut custom_camera = Camera2dBundle::default();
    custom_camera.transform = Transform::from_xyz(0.0, 0.0, 0.0);

    let mut window = windows.single_mut();
    window.resizable = false;
    let screen = window.resolution.physical_size().as_vec2();

    cmd.spawn((custom_camera, CustomCamera));
    cmd.spawn((Player, Mover::new(V2::new((100 ) as f64, 0.0), 40_f64)));

    let mut rng = rand::thread_rng();
    for _ in 1..200 {
        let x: f64 = rng.gen::<f64>() * screen.x as f64; 
        let y: f64 = rng.gen::<f64>() * screen.y as f64;
        let mass: f64 = rng.gen::<f64>() * 10f64; 
        cmd.spawn((Ufo, Mover::new(V2::new(x, y), mass)));
    }
}

fn update(
    windows: Query<&Window>,
    mut query: Query<&mut Mover>,//, With<Player>>,
    mut painter: ShapePainter,
) {
    let start_pos = painter.transform;
    let window = &windows.single();
    for mut mover in &mut query {
        painter.transform = start_pos;
        let mass = mover.mass();
    
        mover.apply_force(&Force::gravity(mass, 0.05));
        if mover.contact(&window) {
            let friction = Friction::new(&mover.vel, 0.1);
             mover.apply_force(&friction.f);
        }
        mover.check_boundary(&window);
    
        mover.update();
        mover.show(&mut painter);
    }
}


fn main() {
    App::new()
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(Shape2dPlugin::default())
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(
            Update,
            (
                quit_game.run_if(input_just_pressed(KeyCode::KeyQ)),
                keyboard_input_system,
                toggle_resolution,
                on_resize,
                update,
            ),
        )
        .run();
}


