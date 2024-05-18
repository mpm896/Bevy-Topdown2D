use bevy::prelude::*;

use crate::AppState;
use crate::TILE_SIZE;

pub struct AsciiPlugin;

#[derive(Resource)]
pub struct AsciiSheet {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Preload), load_ascii);
    }
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) -> Entity {
    assert!(index < 256, "Index out of ASCII range!");

    commands
        .spawn(SpriteSheetBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                color: color,
                ..default()
            },
            texture: ascii.texture.clone(),
            atlas: TextureAtlas {
                index: index,
                layout: texture_atlas_layout,
                ..default()
            },
            transform: Transform {
                translation: translation,
                ..default()
            },
            ..default()
        })
        .id()
}

fn load_ascii(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image: Handle<Image> = asset_server.load("tilemap/Ascii.png");
    let atlas = TextureAtlasLayout::from_grid(
        Vec2::new(TILE_SIZE, TILE_SIZE),
        16,
        16,
        Some(Vec2::new(9.0, 9.0)),
        None,
    );
    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet {
        texture: image,
        layout: atlas_handle,
    });
}
