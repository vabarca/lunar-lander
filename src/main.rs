use bevy::{input::common_conditions::input_just_pressed, prelude::*, render::camera::ScalingMode};
use bevy_vector_shapes::prelude::*;
use lunar_lander::{cameras::*, forces::*, inputs::*, mover::*, vectors::*};
use rand::prelude::*;

fn setup(mut commands: Commands, mut windows: Query<&mut Window>) {
    let mut custom_camera = Camera2dBundle::default();

    custom_camera.projection.scaling_mode = ScalingMode::FixedVertical(1280.0);
    custom_camera.transform = Transform::from_xyz(0.0, 0.0, 0.0);

    let mut window = windows.single_mut();
    window.resizable = false;
    let screen = window.resolution.physical_size().as_vec2();
    let pos1 = V2::new((100 ) as f64, 0.0);
    info!("{}", screen.x);

    commands.spawn((custom_camera, CustomCamera));
    
    commands.spawn((Player, Mover::new(pos1.clone(), 40_f64)));
    let mut rng = rand::thread_rng();
    for _ in 1..200 {
        let mut y: f64 = rng.gen();
        y = y * screen.y as f64;
        let mut x: f64 = rng.gen(); 
        x = x * screen.x as f64;
        let mut mass: f64 = rng.gen(); 
        mass = mass * 10f64;
        commands.spawn((Ufo, Mover::new(V2::new(x, y), mass)));
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


