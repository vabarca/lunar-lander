use bevy::{input::common_conditions::input_just_pressed, prelude::*, sprite::Wireframe2dPlugin};
use lunar_lander::{cameras::*, corners::*, forces::*, inputs::*, mover::*};

fn setup(
    mut cmd: Commands,
    mut windows: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut window = windows.single_mut();
    window.resizable = false;

    let screen = window.resolution.physical_size().as_vec2();
    let rect =  Rect::new(0.0, 0.0, screen.x, screen.y);

    spawn_cameras(&mut cmd, &rect);
    spawn_corners(&mut cmd, &rect, &mut meshes, &mut materials);
    spawn_player(&mut cmd, &rect, &mut meshes, &mut materials);
    spawn_ufos(&mut cmd, &rect, &mut meshes, &mut materials);
}

fn update(
    mut mover_query: Query<(&mut Mover, &mut Transform)>
) {
    for (mut mover, mut transform) in &mut mover_query {
        let mass = mover.mass();

        mover.apply_force(&Force::gravity(mass, 0.05));
        mover.check_surface(0.1);
        mover.check_boundary();
        mover.update();
        mover.show(&mut transform);
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
                mouse_input_system,
            ),
        )
        .run();
}
