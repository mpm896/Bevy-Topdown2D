use bevy::{
    asset::io::gated::GateOpener,
    asset::LoadedFolder,
    render::texture::ImageSampler,
    prelude::*,
    ecs::query,
    render::texture
};
use crate::components::{Player, Velocity, Movable};
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
        .add_systems(Update, player_spawn_system.run_if(in_state(AppState::InGame)))
        .add_systems(Update, player_keyboard_event_system.run_if(in_state(AppState::InGame)))
        .add_systems(Update, player_fire_system.run_if(in_state(AppState::InGame)));
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
    
    let player_info = Vec::new();

    // Create Player texture atlas
    for i in 0..game_textures.player.len() {
        let loaded_foler: &LoadedFolder = loaded_folders.get(game_textures.player[i].id()).unwrap();
        let (texture_atlas, texture) = create_texture_atlas(
            loaded_foler,
            None,
            Some(ImageSampler::nearest()),
            &mut textures
        );
        let atlas_handle = texture_atlases.add(texture_atlas);
        game_textures.player_atlas.push(atlas_handle.clone());

        player_info.push(texture);
        player_info.push(atlas_handle.clone());

        if i == 0 {
            commands.spawn( SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2., 1.),
                    ..default()
                },
                texture,
                atlas: TextureAtlas {
                    index: 1,
                    layout: atlas_handle
                },
                ..default()
            })
            .insert(Player)
            .insert(Velocity { x: 0., y: 0. })
            .insert(Movable { auto_despawn: false });
        }
    }

/*
    // Get texture from TextureAtlasLayout and index
    let folder = loaded_folders.get(game_textures.player[0].id()).unwrap();
    let handle = folder.handles[0].clone();
    let texture_id = handle.id().typed_unchecked::<Image>();
    let texture = Handle::Weak(texture_id);
    let atlas_handle = game_textures.player_atlas[0].clone();

    // Spawn the idle sprite
    commands.spawn( SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2., 0.),
            ..default()
        },
        texture: texture,
        atlas: TextureAtlas {
            index: 2,
            layout: atlas_handle
        },

        ..default()
    })
    .insert(Player)
    .insert(Velocity { x: 0., y: 0. })
    .insert(Movable { auto_despawn: false });
*/
}



fn player_fire_system(
    mut commands: Commands,
    kb: Res<ButtonInput<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            // Get player location
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);

            // Spawn laser at player location
            commands.spawn(SpriteBundle {
                texture: game_textures.player_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::new(LASER_SCALE, LASER_SCALE, 0.),
                    ..default()
                },
                ..default()
            })
            .insert(Velocity { x: 0., y: 2. })
            .insert(Movable { auto_despawn: true });

        }
    }

}

// For every velocity component with the player component, change the velocity based on keyboard input
fn player_keyboard_event_system(
    kb: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    println!("{:?}", kb);
    if let Ok(mut velocity) = query.get_single_mut() {  // get_single_mut() to get a mutable reference when you know there is ONLY one
        velocity.x = if kb.pressed(KeyCode::ArrowLeft) {
            -1.
        } else if kb.pressed(KeyCode::ArrowRight) {
            1.
        } else { 0. };

        velocity.y = if kb.pressed(KeyCode::ArrowDown) {
            -1.
        } else if kb.pressed(KeyCode::ArrowUp) {
            1.
        } else { 0. };
    }
} 
