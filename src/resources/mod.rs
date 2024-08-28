use bevy::prelude::*;

pub mod run_state;
pub mod world;

pub mod map;

#[derive(Resource)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}
