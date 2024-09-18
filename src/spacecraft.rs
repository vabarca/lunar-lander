use bevy::prelude::*;

#[derive(Component)]
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

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

