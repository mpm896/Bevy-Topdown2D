use bevy::prelude::Component;

// Common components
#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

// Movable component to despawn when going out of bounds
#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool
}

#[derive(Component, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}


#[derive(Component)]
pub struct Player;  // Just used as a marker

