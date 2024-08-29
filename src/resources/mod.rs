use bevy::prelude::*;

#[derive(Resource)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}
