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

fn update(mut obj_query: Query<(&mut Body, &mut Transform)>) {
    let mut iter = obj_query.iter_combinations_mut();

    info!("------------------");

    while let Some([(mut body1, _transform1), (mut body2, _transform2)]) = iter.fetch_next() {
        body1.be_attracted(&body2);
        body2.be_attracted(&body1);
        info!("{} {}", body1.name, body2.name)
    }

    for (mut body, mut transform) in &mut obj_query {
        body.update();
        body.show(&mut transform);
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
