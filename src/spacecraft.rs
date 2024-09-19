use bevy::prelude::*;

/// This will be used to identify the main player entity
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ufo;

#[derive(Component)]
pub struct SpacecraftName(pub String);

#[derive(Bundle)]
pub struct Spacecraft{
    name: SpacecraftName,
    sprite: SpriteBundle,
}

impl Spacecraft{
    pub fn new(name: String, sprite: SpriteBundle) -> Self {
        Self {
            name: SpacecraftName(name),
            sprite
        }
    }
}

