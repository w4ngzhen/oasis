use crate::components::field_of_vision::FieldOfVision;
use crate::components::position_2d::Position2d;
use crate::components::role::Player;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::core_module::{map_idx, GAME_MAP_TILE_HEIGHT, GAME_MAP_TILE_WIDTH};
use crate::utils::fov_utils::calc_fov;
use bevy::prelude::*;

pub fn fov(
    mut mb: ResMut<GameMapBuilder>,
    mut views_query: Query<((Entity, &Position2d), &mut FieldOfVision)>,
    query_player: Query<&Player>,
) {
    views_query.iter_mut().filter(|(_, fov)| fov.is_dirty_data).for_each(
        |((entity, pos), mut fov)| {
            let visible_tiles = calc_fov(
                &pos,
                fov.range,
                (GAME_MAP_TILE_WIDTH, GAME_MAP_TILE_HEIGHT),
                |x: i32, y: i32| mb.game_map.is_tile_opacity(&Position2d::new(x, y)),
            );
            fov.visible_tiles = visible_tiles;
            fov.is_dirty_data = false;
            // remember tiles when the player visited.
            if let Ok(_) = query_player.get(entity) {
                fov.visible_tiles.iter().for_each(|pos| {
                    let idx = map_idx(pos.x, pos.y);
                    mb.game_map.visited_tiles[idx] = true;
                })
            }
        },
    );
}
