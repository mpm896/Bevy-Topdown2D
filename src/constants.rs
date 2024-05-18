// asset constants
// For assets, bevy already assumes in an 'assets' directory
pub const PLAYER_SPRITE_FRONT: &str =
    "tiny-RPG-forest-files/PNG/sprites/hero/idle/hero-idle-front/hero-idle-front.png";
pub const PLAYER_SPRITE_BACK: &str =
    "tiny-RPG-forest-files/PNG/sprites/hero/idle/hero-idle-back/hero-idle-back.png";
pub const PLAYER_SPRITE_SIDE: &str =
    "tiny-RPG-forest-files/PNG/sprites/hero/idle/hero-idle-side/hero-idle-side.png";
pub const PLAYER_SIZE: (f32, f32) = (144., 75.);

pub const LASER_SPRITE: &str = "laser_a_01.png";
pub const LASER_SIZE: (f32, f32) = (9., 54.);
pub const LASER_SCALE: f32 = 0.2;

// Game constants
pub const TIME_STEP: f32 = 1. / 60.; // 60 fps
pub const BASE_SPEED: f32 = 100.;
pub const MARGIN: f32 = 200.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

// Tilemap constants
pub const TILE_SIZE: f32 = 32.;
