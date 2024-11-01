use bevy::{prelude::*, sprite::Wireframe2dConfig, window::WindowResized};

#[derive(Component)]
pub struct MainCamera;

/// Marker component for the text that displays the current resolution.
#[derive(Component)]
pub struct ResolutionText;

/// Stores the various window-resolutions we can select between.
#[derive(Resource)]
pub struct ResolutionSettings {
    pub large: Vec2,
    pub medium: Vec2,
    pub small: Vec2,
}

// Spawns the UI
pub fn setup_ui(mut cmd: Commands) {
    // Node that fills entire background
    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        // Text where we display current resolution
        root.spawn((
            TextBundle::from_section(
                "Resolution",
                TextStyle {
                    font_size: 50.0,
                    ..default()
                },
            ),
            ResolutionText,
        ));
    });
}

/// This system shows how to request the window to a new resolution
pub fn toggle_resolution(
    keys: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
    //mut transform_query: Query<&mut Transform, With<MainCamera>>,
) {
    //let mut transform = transform_query.single_mut();
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Digit1) {
        let res = resolution.small;
        window.resolution.set(res.x, res.y);
      //  transform.translation = Vec3::new(res.x/2.0, res.y/2.0, 0.0);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        let res = resolution.medium;
        window.resolution.set(res.x, res.y);
        //transform.translation = Vec3::new(res.x/2.0, res.y/2.0, 0.0);
    }
    if keys.just_pressed(KeyCode::Digit3) {
        let res = resolution.large;
        window.resolution.set(res.x, res.y);
       // transform.translation = Vec3::new(res.x/2.0, res.y/2.0, 0.0);
    }

}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
pub fn on_resize(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.read() {
        // When resolution is being changed
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    }
}

pub fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

pub fn spawn_cameras(
    cmd: &mut Commands, 
){
    cmd.spawn((Camera2dBundle::default(), MainCamera));
}

