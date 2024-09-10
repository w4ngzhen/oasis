use crate::components::position_2d::Position2d;

pub mod game_hud;
pub mod game_map;
pub mod player;

pub mod prelude {}
pub const GAME_MAP_TILE_WIDTH: i32 = 80;
pub const GAME_MAP_TILE_HEIGHT: i32 = 80;
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * GAME_MAP_TILE_WIDTH) + x) as usize
}

pub fn map_idx_by_pos(pos: &Position2d) -> usize {
    map_idx(pos.x, pos.y)
}
