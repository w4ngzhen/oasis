pub mod game_log;

use crate::components::position_2d::Position2d;
use bevy::prelude::*;

#[derive(Resource)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

/// 地图相机中心位置是那个tile位置
#[derive(Resource)]
pub struct MapCameraCenter(pub Option<Position2d>);

///
#[derive(Resource)]
pub struct TileSize(pub f32);



