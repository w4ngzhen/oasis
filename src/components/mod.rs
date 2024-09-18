use crate::components::position_2d::Position2d;
use bevy::prelude::*;

pub mod attributes;
pub mod role;
pub mod attack;
pub mod bundles;
pub mod field_of_vision;
pub mod item;
pub mod name;
pub mod position_2d;

/// Map tile tag
#[derive(Component)]
pub struct MapTileElement {
    pub color: Color,
    pub is_background: bool,
}

/// 标识能否被绘制
#[derive(Component)]
pub struct MapTileDrawable;

/// 用来呈现在地图镜头中心的元素的位置
#[derive(Component)]
pub struct CenterTilePosition(pub Position2d);

/// 移动组件
/// 该组件会在 移动系统 中消费
#[derive(Component, Clone, Copy, Debug)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position2d,
}

/// 游戏地图摄像机，只关注地图场景渲染
#[derive(Component, Clone, Copy)]
pub struct GameMapCamera;

/// 游戏HUD模块Camera
#[derive(Component, Clone, Copy)]
pub struct GameHudCamera;

/// 地图选择组件
#[derive(Component, Clone, Copy)]
pub struct MapPickCursor;

#[derive(Component, Clone, Copy)]
pub struct WantsToDestroy(pub Entity);
#[derive(Component, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Naming(pub(crate) String);
#[derive(Component, Clone)]
pub struct Description(pub String);

#[derive(Component, Clone)]
pub struct Weapon;

#[derive(Component, Clone)]
pub struct Equipment {
    head: Option<Entity>,
    body: Option<Entity>,
    left_hand: Option<Entity>,
    right_hand: Option<Entity>,
    foot: Option<Entity>,
}
