use bevy::prelude::*;
use crate::components::position::Position;

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