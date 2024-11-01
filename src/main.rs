use bevy::{input::common_conditions::input_just_pressed, prelude::*, sprite::Wireframe2dPlugin};
use bevy_vector_shapes::prelude::*;
use lunar_lander::{cameras::*, forces::*, inputs::*, mover::*, shapes::*};

fn setup(
    mut cmd: Commands, 
    mut windows: Query<&mut Window>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>
) {
    let mut window = windows.single_mut();
    
    spawn_cameras(&mut cmd);
    spawn_corners(&mut cmd, meshes, materials, &mut window);
    spawn_player(&mut cmd);
    spawn_ufos(&mut cmd, &mut window);
}

fn update(
    windows: Query<&Window>,
    mut mover_query: Query<&mut Mover>,
    mut painter: ShapePainter,
) {
    let start_pos = painter.transform;
    let window = &windows.single();
    for mut mover in &mut mover_query {
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
            medium: Vec2::new(1280.0, 720.0),
            small: Vec2::new(800.0, 600.0),
        })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(Wireframe2dPlugin)
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(
            Update,
            (
                quit_game.run_if(input_just_pressed(KeyCode::KeyQ)),
                keyboard_input_system,
                toggle_resolution,
                on_resize,
                update,
                toggle_wireframe,
            ),
        )
        .run();
}


