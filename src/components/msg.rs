use crate::components::position_2d::Position2d;
use bevy::prelude::*;

/// 移动组件
/// 该组件会在 移动系统 中消费
#[derive(Component, Clone, Copy, Debug)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position2d,
}

/// 发起攻击
#[derive(Component, Clone, Copy)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub target: Option<Entity>,
}

/// 地图上Pick模式下试图查询该位置的元素
#[derive(Component, Clone, Copy)]
pub struct MapWantsToPick {
    pub position: Position2d,
}

/// 准备销毁某个实体
#[derive(Component, Clone, Copy)]
pub struct WantsToDestroy(pub Entity);
