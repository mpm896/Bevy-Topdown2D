use bevy::math::f32;
use bevy::{
    asset::io::gated::GateOpener,
    asset::LoadedFolder,
    render::texture::ImageSampler,
    prelude::*,
    ecs::query,
    render::texture
};
use crate::components::{Player, Velocity, Movable, Direction};
use crate::{
    GameTextures,
    WinSize, 
    PLAYER_SIZE, 
    PLAYER_SPRITE_FRONT,
    LASER_SCALE,
    TIME_STEP, 
    BASE_SPEED,
    AppState,
    create_texture_atlas,
    create_sprite_from_atlas,
};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::InGame), player_spawn_system)
        .add_systems(Update, player_keyboard_event_system.run_if(in_state(AppState::InGame)))
        .add_systems(Update, player_fire_system.run_if(in_state(AppState::InGame)))
        .add_systems(Update, change_player_direction_system.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub idle_sprite: SpriteBundle,
    pub velocity: Velocity,
    pub movable: Movable,
}


fn player_spawn_system(
    mut commands: Commands,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut game_textures: ResMut<GameTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut textures: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>
) {
    // Get the window size
    let bottom = -win_size.h / 2.;

    // Create Player texture atlas
    for i in 0..game_textures.player_folders.len() {
        let loaded_foler: &LoadedFolder = loaded_folders.get(game_textures.player_folders[i].id()).unwrap();
        let (texture_atlas, texture) = create_texture_atlas(
            loaded_foler,
            None,
            Some(ImageSampler::nearest()),
            &mut textures
        );
        let atlas_handle = texture_atlases.add(texture_atlas);
        game_textures.player_atlas.push(atlas_handle.clone());
        game_textures.player_textures.push(texture.clone());
    }

    // Spawn the player
    commands.spawn( SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2., 0.),
            ..default()
        },
        texture: game_textures.player_textures[0].clone(),
        atlas: TextureAtlas {
            index: 0,
            layout: game_textures.player_atlas[0].clone()
        },
        sprite: Sprite {
            flip_x: false,
            ..default()
        },
        ..default()
    })
    .insert(Player)
    .insert(Velocity { x: 0., y: 0. })
    .insert(Movable { auto_despawn: false })
    .insert(Direction::Down);
}


// Change the player sprite based on direction
fn change_player_direction_system(
    mut query: Query<(&mut TextureAtlas, &Direction), With<Player>>
) {
    if let Ok((mut sprite, direction)) = query.get_single_mut() {
        match direction {
            Direction::Up => sprite.index = 2,
            Direction::Down => sprite.index = 1,
            Direction::Left => sprite.index = 0,
            Direction::Right => sprite.index = 0
        }
    }
}

// Fire the laser (or swing the weapon, will update later)
fn player_fire_system(
    mut commands: Commands,
    kb: Res<ButtonInput<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<(&Transform, &Direction), With<Player>>
) {
    if let Ok((player_tf, player_dir)) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            // Get player location
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);

            // Get player direction
            let (dx, dy, rot): (f32, f32, f32) = match player_dir {
                Direction::Up => (0., 1., 0.),
                Direction::Down => (0., -1., 180.),
                Direction::Left => (-1., 0., 90.),
                Direction::Right => (1., 0., -90.),
            };
            
            // Spawn laser at player location with proper direction and velocity
            commands.spawn(SpriteBundle {
                texture: game_textures.player_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    rotation: Quat::from_rotation_z(rot.to_radians()),
                    scale: Vec3::new(LASER_SCALE, LASER_SCALE, 0.),
                    ..default()
                },
                ..default()
            })
            .insert(Velocity { x: 2. * dx, y: 2. * dy })
            .insert(Movable { auto_despawn: true });

        }
    }

}


// For every velocity component and direction component with the player component, 
// change the velocity and direction based on keyboard input
fn player_keyboard_event_system(
    kb: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Direction), With<Player>>
) {
    if let Ok((mut velocity, mut direction)) = query.get_single_mut() {  // get_single_mut() to get a mutable reference when you know there is ONLY one
        if kb.pressed(KeyCode::ArrowLeft) {
            *direction = Direction::Left;
            velocity.x = -1.;
        } else if kb.pressed(KeyCode::ArrowRight) {
            *direction = Direction::Right;
            velocity.x = 1.;
        } else {
            velocity.x = 0.;
        }

        if kb.pressed(KeyCode::ArrowDown) {
            *direction = Direction::Down;
            velocity.y = -1.;
        } else if kb.pressed(KeyCode::ArrowUp) {
            *direction = Direction::Up;
            velocity.y = 1.;
        } else {
            velocity.y = 0.;
        }
    }
} 
