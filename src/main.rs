use bevy::{input::common_conditions::input_just_pressed, prelude::*, sprite::Wireframe2dPlugin};
use lunar_lander::{attractor::*, cameras::*, corners::*, inputs::*, mover::*};

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
    spawn_attractor(&mut cmd, &rect, &mut meshes, &mut materials);
    spawn_ufos(&mut cmd, &rect, &mut meshes, &mut materials);
}

fn update(
    mut players_query: Query<(&mut Mover, &mut Transform), With<Player>>,
    attractors_query: Query<&Attractor>
) {
    let (mut player, mut player_transform) = players_query.single_mut();
    let attractor = attractors_query.single();
    
    let attration = attractor.attract(&player);

    player.apply_force(attration);
    player.update();
    player.show(&mut player_transform);
}

fn update_ufos(
    mut ufos_query: Query<(&mut Mover, &mut Transform), With<Ufo>>,
    attractors_query: Query<&Attractor>
) {
    let attractor = attractors_query.single();

    for (mut ufo, mut ufo_transform) in &mut ufos_query {

        let attration = attractor.attract(&ufo);

        ufo.apply_force(attration);
        ufo.update();
        ufo.show(&mut ufo_transform);
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
                update_ufos,
                toggle_wireframe,
            ),
        )
        .run();
}
