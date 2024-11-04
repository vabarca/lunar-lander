use bevy::math::Rect;
use rand::Rng;
use crate::vectors::V2;


pub fn random_coordinates(rect: &Rect) -> V2 {
    V2::new(random_x_coordinate(rect), random_y_coordinate(rect))
}

pub fn middle_coordinates(rect: &Rect) -> V2 {
    let diff = rect.max - rect.min;
    V2::new(diff.x as f64/2.0, diff.y as f64/2.0)
}

pub fn random_x_coordinate(rect: &Rect) -> f64 {
    let mut rng = rand::thread_rng();
    let diff = rect.max - rect.min;
    rng.gen::<f64>() * diff.x as f64 + rect.min.x as f64
}

pub fn random_y_coordinate(rect: &Rect) -> f64 {
    let mut rng = rand::thread_rng();
    let diff = rect.max - rect.min;
    rng.gen::<f64>() * diff.y as f64 + rect.min.y as f64
}
