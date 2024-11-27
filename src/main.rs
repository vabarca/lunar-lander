use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use lunar_lander::{bodies::*, cameras::*, inputs::*};

fn setup(mut cmd: Commands, mut windows: Query<&mut Window>, asset_server: Res<AssetServer>) {
    let mut window = windows.single_mut();
    window.resizable = false;

    let screen = window.resolution.physical_size().as_vec2();
    let rect = Rect::new(0.0, 0.0, screen.x, screen.y);

    spawn_cameras(&mut cmd, &rect);
    spawn_player(&mut cmd, &rect, &asset_server);
    spawn_ufos(&mut cmd, &rect, &asset_server);
    spawn_attractor(&mut cmd, &rect, &asset_server);
}

fn update(
    mut players_query: Query<(&mut Body, &mut Transform), With<Player>>,
    mut ufos_query: Query<
        (&mut Body, &mut Transform),
        (With<Ufo>, Without<Player>, Without<Attractor>),
    >,
    mut attractors_query: Query<
        (&mut Body, &mut Transform),
        (With<Attractor>, Without<Player>, Without<Ufo>),
    >,
) {
    let (player, player_transform) = players_query.single_mut();
    let (attractor, attractor_transform) = attractors_query.single_mut();

    let mut bodies = Vec::new();
    let mut bodies_copy = Vec::new();
    let mut tranforms = Vec::new();

    for (ufo, tranform) in &mut ufos_query {
        bodies_copy.push(ufo.clone());
        bodies.push(ufo);
        tranforms.push(tranform);
    }

    bodies.push(attractor);
    tranforms.push(attractor_transform);
    bodies.push(player);
    tranforms.push(player_transform);


    let mut i :usize = 0;
    for mut fr in bodies{
        for sc in &bodies_copy{
            if fr.mass != sc.mass {
                fr.be_attracted(&sc);
            }
        }
        fr.update();
        fr.show(&mut tranforms[i]);
        i +=1 ;
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
