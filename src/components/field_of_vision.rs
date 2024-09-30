use crate::components::position_2d::Position2d;
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct FieldOfVision {
    /// 玩家此时能看到的视野位置
    pub visible_positions: HashSet<Position2d>,
    /// 视野范围
    pub range: i32,
    /// 是否当前视野位置已经是脏数据，脏数据情况下，会重新计算视野
    pub is_dirty_data: bool,
}

impl Default for FieldOfVision {
    fn default() -> Self {
        Self {
            visible_positions: HashSet::new(),
            range: 8,
            is_dirty_data: true,
        }
    }
}
