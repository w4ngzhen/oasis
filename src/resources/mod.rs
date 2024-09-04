use crate::components::position::Position;
use bevy::prelude::*;

#[derive(Resource)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct MapRenderCenterPosition(pub Option<Position>);
