use bevy::{input::common_conditions::input_just_pressed, prelude::*, sprite::Wireframe2dPlugin};
use lunar_lander::{cameras::*, corners::*, inputs::*, bodies::*};

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
    spawn_attractor(&mut cmd, &rect, &mut meshes, &mut materials);
}

fn update(
    mut players_query: Query<(&mut Body, &mut Transform), With<Player>>,
    mut ufos_query: Query<(&mut Body, &mut Transform), (With<Ufo>, Without<Player>, Without<Attractor>)>,
    attractors_query: Query<&Body, (With<Attractor>, Without<Player>, Without<Ufo>)>
) {
    let (mut player, mut player_transform) = players_query.single_mut();
    let attractor = attractors_query.single();

    player.be_attracted(attractor);
    player.update();
    player.show(&mut player_transform);

    for (mut ufo, mut ufo_transform) in &mut ufos_query {

        ufo.be_attracted(attractor);
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
                on_resize,
                update,
                mouse_input_system,
            ),
        )
        .run();
}
