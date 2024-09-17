use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Spacecraft{
    name: String,
    x: usize,
    y: usize,  
    sprite: SpriteBundle,
}

impl Spacecraft{
    pub fn new(name: String, x: usize, y: usize, sprite: SpriteBundle) -> Self {
        Self {
            x,
            y,
            name,
            sprite,
        }
    }
}

