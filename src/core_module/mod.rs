pub mod game_hud;
pub mod game_map;
pub mod player;

pub mod prelude {}
pub const GAME_MAP_TILE_WIDTH: u64 = 12;
pub const GAME_MAP_TILE_HEIGHT: u64 = 12;
pub const TILE_SIZE: f32 = 16.;
pub fn map_idx(x: u64, y: u64) -> usize {
    ((y * GAME_MAP_TILE_WIDTH) + x) as usize
}
