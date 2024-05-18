use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ascii::{spawn_ascii_sprite, AsciiSheet};
use crate::components::TileCollider;
use crate::{AppState, WinSize, TILE_SIZE};
use bevy::{prelude::*, transform::commands};

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Setup), spawn_ascii_map);
    }
}

fn spawn_ascii_map(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    win_size: Res<WinSize>,
) {
    let file = File::open("assets/tilemap/Ascii.txt").expect("File not found");
    let mut tiles: Vec<Entity> = Vec::new();

    // Tile size, rows, columns, and padding are hardcoded here and specific to this tilemap image
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(9., 9.), 16, 16, Some(Vec2::new(2., 2.)), None);
    let texture_atlas_layout = texture_atlases.add(layout);

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(
                        x as f32 * TILE_SIZE - win_size.w / 2. + 16.,
                        y as f32 * TILE_SIZE - win_size.h / 2. + 8.,
                        0.,
                    ),
                    texture_atlas_layout.clone(),
                );
                if char == '#' {
                    commands.entity(tile).insert(TileCollider); // Inserts a tilecollider component to this entity
                }
                tiles.push(tile);
            }
        }
    }
    commands
        .spawn(SpatialBundle {
            // Use SpatialBundle instead of TransformBundle becaause of difference in InheritedVisibility between parent and child
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            ..default()
        })
        .insert(Name::new("Map"))
        .push_children(&tiles);
}
