use crate::components::{AnimationIndices, AnimationTimer, Direction, Movable, Player, TileCollider, Velocity};
use crate::constants::{BASE_SPEED, LASER_SCALE, PLAYER_SIZE, TIME_STEP};
use crate::resources::{GameTextures, WinSize};
use crate::{create_sprite_from_atlas, create_texture_atlas, collision_check_system, AppState};
use bevy::{
    asset::io::gated::GateOpener, asset::LoadedFolder, ecs::query, prelude::*, render::texture,
    render::texture::ImageSampler,
};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), player_spawn_system)
            .add_systems(
                Update,
                player_keyboard_event_system.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                player_fire_system.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                player_animation_system.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                update_player_animation_texture_system.run_if(in_state(AppState::InGame)),
            );
            //.add_systems(Update, player_tile_collision_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut game_textures: ResMut<GameTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut textures: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>,
) {
    // Get the window size
    let bottom = -win_size.h / 2.;

    // Create Player texture atlas
    for i in 0..game_textures.player_folders.len() {
        let loaded_foler: &LoadedFolder = loaded_folders
            .get(game_textures.player_folders[i].id())
            .unwrap();
        let (texture_atlas, texture) = create_texture_atlas(
            loaded_foler,
            None,
            Some(ImageSampler::nearest()),
            &mut textures,
        );
        let atlas_handle = texture_atlases.add(texture_atlas);
        game_textures.player_atlas.push(atlas_handle.clone());
        game_textures.player_textures.push(texture.clone());
    }

    // Create animation indices component
    let animation_indices = AnimationIndices { first: 0, last: 2 };

    // Spawn the player
    commands
        .spawn((
            SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2., 0.),
                    ..default()
                },
                texture: game_textures.player_textures[0].clone(),
                atlas: TextureAtlas {
                    index: animation_indices.first,
                    layout: game_textures.player_atlas[0].clone(),
                },
                sprite: Sprite {
                    flip_x: false,
                    ..default()
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(Player)
        .insert(Velocity { x: 0., y: 0. })
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(Direction::Down);
}

// Use animation textures if moving
fn player_animation_system(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlas,
            &Velocity,
        ),
        With<Player>,
    >,
) {
    // If velocity is 0, don't animate
    if let Ok((indices, mut timer, mut atlas, velocity)) = query.get_single_mut() {
        if velocity.x == 0. && velocity.y == 0. {
            return;
        }

        // Update the timer and index
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            }
        }
    }
}

// Update the TextureAtlasLayout and animation indices depending on direction
fn update_player_animation_texture_system(
    mut query: Query<(
            &mut AnimationIndices,
            &mut TextureAtlas,
            &mut Handle<Image>,
            &Direction,
            &Velocity,
        ),
        With<Player>>,
    mut game_textures: ResMut<GameTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Ok((mut indices, mut atlas, mut image, direction, velocity)) = query.get_single_mut() {
        let moving: bool = velocity.x != 0. || velocity.y != 0.;
        let indices_moving = AnimationIndices { first: 0, last: 5 };
        match direction {
            Direction::Up => {
                if moving {
                    if *image == game_textures.player_textures[1].clone() {
                        return;
                    }
                    *indices = indices_moving;
                    *image = game_textures.player_textures[1].clone();
                    *atlas = TextureAtlas {
                        index: indices.first,
                        layout: game_textures.player_atlas[1].clone(),
                    };
                } else {
                    *image = game_textures.player_textures[0].clone();
                    *atlas = TextureAtlas {
                        index: 2,
                        layout: game_textures.player_atlas[0].clone(),
                    };
                }
            }
            Direction::Down => {
                if moving {
                    if *image == game_textures.player_textures[2].clone() {
                        return;
                    }
                    *indices = indices_moving;
                    *image = game_textures.player_textures[2].clone();
                    *atlas = TextureAtlas {
                        index: indices.first,
                        layout: game_textures.player_atlas[2].clone(),
                    };
                } else {
                    *image = game_textures.player_textures[0].clone();
                    *atlas = TextureAtlas {
                        index: 1,
                        layout: game_textures.player_atlas[0].clone(),
                    };
                }
            }
            _ => {
                if moving {
                    if *image == game_textures.player_textures[3].clone() {
                        return;
                    }
                    *indices = indices_moving;
                    *image = game_textures.player_textures[3].clone();
                    *atlas = TextureAtlas {
                        index: indices.first,
                        layout: game_textures.player_atlas[3].clone(),
                    };
                } else {
                    *image = game_textures.player_textures[0].clone();
                    *atlas = TextureAtlas {
                        index: 0,
                        layout: game_textures.player_atlas[0].clone(),
                    };
                }
            }
        }
    }
}

// Fire the laser (or swing the weapon, will update later)
fn player_fire_system(
    mut commands: Commands,
    kb: Res<ButtonInput<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<(&Transform, &Direction), With<Player>>,
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
            commands
                .spawn(SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.),
                        rotation: Quat::from_rotation_z(rot.to_radians()),
                        scale: Vec3::new(LASER_SCALE, LASER_SCALE, 0.),
                        ..default()
                    },
                    ..default()
                })
                .insert(Velocity {
                    x: 2. * dx,
                    y: 2. * dy,
                })
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
    if let Ok((mut velocity, mut direction)) = query.get_single_mut() {
        // get_single_mut() to get a mutable reference when you know there is ONLY one
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

// Check for collisions with tiles
fn player_tile_collision_system(
    player_query: Query<(&Transform, &Direction), With<Player>>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
) {
    if let Ok((player_tf, player_dir)) = player_query.get_single() {
        let collision = collision_check_system(player_tf.translation, &wall_query);
        if collision {
            println!("Collision!");
        }
    }
}
