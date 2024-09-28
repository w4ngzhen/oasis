use crate::components::map_element::MapElement;
use crate::components::position_2d::Position2d;
use bevy::prelude::*;

pub mod attributes;
pub mod bundles;
pub mod camera;
pub mod field_of_vision;
pub mod item;
pub mod map_element;
pub mod msg;
pub mod position_2d;
pub mod role;

/// Map tile tag
#[derive(Component)]
pub struct MapTileElement {
    pub color: Color,
}

impl Default for MapTileElement {
    fn default() -> Self {
        Self { color: Color::WHITE }
    }
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
