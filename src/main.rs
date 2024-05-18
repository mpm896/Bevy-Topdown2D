#![allow(unused)] // Silence warning for unused code while exploring

use std::iter;

use ascii::AsciiPlugin;
use bevy::{
    asset::LoadedFolder, 
    ecs::query, 
    input::keyboard::KeyboardInput,
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    render::texture::ImageSampler, 
    transform::commands, ui::update
};
use components::{Direction, Movable, Player, TileCollider, Velocity};
use constants::*;
use player::PlayerPlugin;
use resources::{GameTextures, RpgSpriteFolder, WinSize};
use tilemap::TileMapPlugin;

pub mod ascii;
pub mod components; // Needs to be made public so other files can use it!
pub mod constants;
mod player;
pub mod resources;
mod tilemap;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Preload,
    Setup,
    InGame,
    Finished,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Topdown RPG".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins(PlayerPlugin)
        .add_plugins(AsciiPlugin)
        .add_plugins(TileMapPlugin)
        .add_systems(OnEnter(AppState::Preload), load_player_sprites)
        .add_systems(OnEnter(AppState::Preload), get_winsize)
        .add_systems(Update, check_textures.run_if(in_state(AppState::Preload)))
        .add_systems(OnEnter(AppState::Setup), setup)
        .add_systems(
            Update,
            (movable_system, sprite_flip_system).run_if(in_state(AppState::InGame)),
        )
        .run();
}

// Insert window size resource
fn get_winsize(mut commands: Commands, mut windows: Query<&mut Window>) {
    // Get the window size
    let window = windows.get_single().unwrap();
    let (win_h, win_w) = (window.height(), window.width());

    // Can now insert window size as a resourse
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);
}

// startup system for 2D
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Spawn the 2d camera
    commands.spawn(Camera2dBundle::default());

    // Advance the AppState
    next_state.set(AppState::InGame);
}

// Load all player textures into a Vec of handles
fn load_player_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_textures = GameTextures {
        player_folders: vec![
            asset_server.load_folder("tiny-RPG-forest-files/PNG/sprites/hero/idle"),
            asset_server.load_folder("tiny-RPG-forest-files/PNG/sprites/hero/walk/hero-walk-back"),
            asset_server.load_folder("tiny-RPG-forest-files/PNG/sprites/hero/walk/hero-walk-front"),
            asset_server.load_folder("tiny-RPG-forest-files/PNG/sprites/hero/walk/hero-walk-side"),
        ],
        player_laser: asset_server.load(LASER_SPRITE),
        ..default()
    };

    commands.insert_resource(game_textures);
}

// Create a texture atlas
fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>) {
    // Build a texture atlas using the individual sprites
    let mut texture_atlas_builder =
        TextureAtlasBuilder::default().padding(padding.unwrap_or_default());
    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an 'Image' asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
    let texture = textures.add(texture);

    // Update the sampling settings of the texture atlas
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture)
}

fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    game_textures: Res<GameTextures>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the AppState once all the sprite handles have been loaded by the asset server
    for event in events.read() {
        for rpg_sprite_folder in game_textures.player_folders.iter() {
            if event.is_loaded_with_dependencies(rpg_sprite_folder) {
                next_state.set(AppState::Setup)
            }
        }
    }
}

// Spawn a sprite from a texture atlas
fn create_sprite_from_atlas(
    commands: &mut Commands,
    translations: (f32, f32, f32),
    sprite_index: usize,
    atlas_handle: Handle<TextureAtlasLayout>,
    texture: Handle<Image>,
) {
    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(translations.0, translations.1, translations.2),
            ..default()
        },
        texture,
        atlas: TextureAtlas {
            index: sprite_index,
            layout: atlas_handle,
        },
        ..default()
    });
}

// For every velocity and transform component together with a player component, change the player position (i.e. translation) based on the updated velocity
fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable), Without<TileCollider>>,
    mut wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>
) {
    // only '&' for read-only access. '&mut' for read-write access
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        // iter_mut() because we're going to mutate the transform
        // Apply velocity to get target position
        let x_delta = velocity.x * TIME_STEP * BASE_SPEED;
        let x_target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
        if !collision_check_system(x_target, &wall_query) {
            transform.translation.x += x_delta;
        };

        let y_delta = velocity.y * TIME_STEP * BASE_SPEED;
        let y_target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
        if !collision_check_system(y_target, &wall_query) {
            transform.translation.y += y_delta;
        };

        // translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        // translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            if transform.translation.y > win_size.h / 2. + MARGIN
                || transform.translation.y < -win_size.h / 2. - MARGIN
                || transform.translation.x > win_size.w / 2. + MARGIN
                || transform.translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn sprite_flip_system(mut query: Query<(&mut Sprite, &Direction), With<Movable>>) {
    for (mut sprite, direction) in query.iter_mut() {
        match direction {
            Direction::Left => sprite.flip_x = true,
            _ => sprite.flip_x = false,
        }
    }
}

fn collision_check_system(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>
) -> bool {
    for wall_transform in wall_query.iter() {
        
        // Get Aabb2d of wall and player
        let wall_rect = Aabb2d::new(
            wall_transform.translation.truncate(),
            Vec2::splat(TILE_SIZE / 2.)
        );
        let player_rect = Aabb2d::new(
            target_player_pos.truncate(),
            Vec2::new(6., 11.)
        );

        let collision = player_rect.intersects(&wall_rect);
        println!("Collision: {}", collision);
        return collision
    }

    false
}

// System to print keyboard events as they come in
fn print_keyboard_events(mut keyboard_events: EventReader<KeyboardInput>) {
    for event in keyboard_events.read() {
        println!("{:?}", event);
    }
}
