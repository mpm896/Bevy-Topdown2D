use bevy::{
    asset::{Handle, LoadedFolder},
    prelude::Resource,
    render::texture::Image,
    sprite::TextureAtlasLayout
};

// Resources
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32
}

#[derive(Resource, Default)]
pub struct GameTextures {  // Instead of needing AssetServer everywhere
    pub player_folders: Vec<Handle<LoadedFolder>>,
    pub player_atlas: Vec<Handle<TextureAtlasLayout>>,
    pub player_textures: Vec<Handle<Image>>,
    pub player_laser: Handle<Image>
}

#[derive(Resource, Debug)]
pub struct RpgSpriteFolder(Handle<LoadedFolder>);