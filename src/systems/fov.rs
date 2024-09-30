use crate::components::field_of_vision::FieldOfVision;
use crate::components::map_element::MapElementProp;
use crate::components::position_2d::Position2d;
use crate::components::role::Player;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::core_module::{GAME_MAP_TILE_HEIGHT, GAME_MAP_TILE_WIDTH};
use crate::utils::fov_utils::calc_fov;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn fov(
    mut mb: ResMut<GameMapBuilder>,
    mut views_query: Query<((Entity, &Position2d), &mut FieldOfVision)>,
    q_ele_prop: Query<(&Position2d, &MapElementProp)>,
    q_player: Query<&Player>,
) {
    views_query.iter_mut().filter(|(_, fov)| fov.is_dirty_data).for_each(
        |((entity, pos), mut fov)| {
            let visible_pos = calc_fov(
                &pos,
                fov.range,
                (GAME_MAP_TILE_WIDTH, GAME_MAP_TILE_HEIGHT),
                |x: i32, y: i32| {
                    // 找到对应位置所有的实体，只要有一个会阻挡视线，则该位置就会阻挡视野
                    q_ele_prop
                        .iter()
                        .filter(|(pos, _)| pos.x == x && pos.y == y)
                        .any(|(_, prop)| prop.is_block_view)
                },
            );
            fov.visible_positions = visible_pos;
            fov.is_dirty_data = false;
            // 如果当前是player在进行视野系统，则补充记录玩家曾经看到过的地方
            if let Ok(_) = q_player.get(entity) {
                fov.visible_positions.iter().for_each(|pos| {
                    mb.game_map.visited_positions.insert(pos.clone());
                })
            }
        },
    );
}
