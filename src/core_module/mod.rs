pub mod game_hud;
pub mod game_map;
pub mod player;

pub mod prelude {}
pub const GAME_MAP_TILE_WIDTH: i32 = 12;
pub const GAME_MAP_TILE_HEIGHT: i32 = 12;
pub const TILE_SIZE: f32 = 16.;
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * GAME_MAP_TILE_WIDTH) + x) as usize
}
