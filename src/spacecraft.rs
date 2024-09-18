use bevy::prelude::*;

/// This will be used to identify the main player entity
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct SpacecraftName(pub String);

#[derive(Bundle)]
pub struct Spacecraft{
    marker: Player,
    name: SpacecraftName,
    sprite: SpriteBundle,
}

impl Spacecraft{
    pub fn new(name: String, sprite: SpriteBundle) -> Self {
        Self {
            marker: Player,
            name: SpacecraftName(name),
            sprite
        }
    }
}

