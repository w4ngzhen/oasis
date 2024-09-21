use crate::components::position_2d::Position2d;
use bevy::prelude::*;

pub mod attributes;
pub mod bundles;
pub mod field_of_vision;
pub mod item;
pub mod position_2d;
pub mod role;
pub mod msg;
pub mod camera;

/// Map tile tag
#[derive(Component)]
pub struct MapTileElement {
    pub color: Color,
    pub is_background: bool,
}

/// 用来呈现在地图镜头中心的元素的位置
#[derive(Component)]
pub struct CenterTilePosition(pub Position2d);

/// 地图选择组件
#[derive(Component, Clone, Copy)]
pub struct MapPickCursor;

#[derive(Component, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Naming(pub(crate) String);
#[derive(Component, Clone)]
pub struct Description(pub String);
