use crate::components::position::Position;
use bevy::prelude::*;

#[derive(Resource)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

/// 地图相机中心位置是那个tile位置
#[derive(Resource)]
pub struct MapCameraCenter(pub Option<Position>);
