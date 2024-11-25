use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use lunar_lander::{cameras::*, inputs::*, bodies::*};

fn setup(
    mut cmd: Commands,
    mut windows: Query<&mut Window>,
    asset_server: Res<AssetServer>
) {
    let mut window = windows.single_mut();
    window.resizable = false;

    let screen = window.resolution.physical_size().as_vec2();
    let rect =  Rect::new(0.0, 0.0, screen.x, screen.y);

    spawn_cameras(&mut cmd, &rect);
    spawn_player(&mut cmd, &rect, &asset_server);
    spawn_ufos(&mut cmd, &rect, &asset_server);
    spawn_attractor(&mut cmd, &rect, &asset_server);
}

fn update(
    mut players_query: Query<(&mut Body, &mut Transform), With<Player>>,
    mut ufos_query: Query<(&mut Body, &mut Transform), (With<Ufo>, Without<Player>, Without<Attractor>)>,
    mut attractors_query: Query<(&mut Body, &mut Transform), (With<Attractor>, Without<Player>, Without<Ufo>)>,
    mut gizmos: Gizmos
) {
    let (player, player_transform) = players_query.single_mut();
    let (attractor, attractor_transform) = attractors_query.single_mut();

    let mut bodies = Vec::new();
    let mut tranforms = Vec::new();

    for (ufo, tranform) in &mut ufos_query {
        bodies.push(ufo);
        tranforms.push(tranform);
    }

    bodies.push(attractor);
    tranforms.push(attractor_transform);
    bodies.push(player);
    tranforms.push(player_transform);


    for i in 0..bodies.len() {
        let (left, right) = bodies.split_at_mut(i + 1);
        let fr = &mut left[i];
        //let mut counter = 0;
        // info!("Iteration: {i}");
        for sc in right {
            fr.be_attracted(sc);
            // info!("   * {counter}");
            //counter += 1;
        }
    }

    for i in 0..bodies.len(){
        bodies[i].update();
        bodies[i].show(&mut gizmos, &mut tranforms[i])
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
