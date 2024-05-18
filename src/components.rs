use bevy::prelude::*;

// Common components
#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// Movable component to despawn when going out of bounds
#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Animation components
#[derive(Component, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

// Player components
#[derive(Component)]
pub struct Player; // Just used as a marker

// Tilemap components
#[derive(Component)]
pub struct TileCollider;

