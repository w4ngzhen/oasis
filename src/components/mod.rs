use crate::components::position::Position;
use bevy::prelude::*;

pub mod attributes;
pub mod role;

pub mod field_of_vision;
pub mod name;
pub mod position;

/// Map tile tag
#[derive(Component)]
pub struct TileElement;

/// 移动组件
/// 该组件会在 移动系统 中消费
#[derive(Component, Clone, Copy)]
pub struct Movement {
    pub entity: Entity,
    pub destination: Position,
}

/// 游戏地图摄像机，只关注地图场景渲染
#[derive(Component, Clone, Copy)]
pub struct GameMapCamera;

/// 游戏UI模块Camera
#[derive(Component, Clone, Copy)]
pub struct GameUiCamera;
