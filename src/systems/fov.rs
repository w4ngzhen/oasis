use crate::components::field_of_vision::FieldOfVision;
use crate::components::position::Position;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::core_module::{GAME_MAP_TILE_HEIGHT, GAME_MAP_TILE_WIDTH};
use crate::utils::fov_utils::calc_fov;
use bevy::prelude::*;

pub fn fov(mb: Res<GameMapBuilder>, mut views_query: Query<(&Position, &mut FieldOfVision)>) {
    views_query.iter_mut().filter(|(_, fov)| fov.is_dirty_data).for_each(|(pos, mut fov)| {
        let visible_tiles = calc_fov(
            &pos,
            fov.range,
            (GAME_MAP_TILE_WIDTH, GAME_MAP_TILE_HEIGHT),
            |x: i32, y: i32| mb.game_map.is_tile_opacity::<Position>(&Position::new(x, y, 0)),
        );
        fov.visible_tiles = visible_tiles;
        fov.is_dirty_data = false;
    });
}
