pub mod map;
pub mod player;
pub mod prelude {}
pub const REGION_TILE_WIDTH: u64 = 80;
pub const REGION_TILE_HEIGHT: u64 = 80;
pub const TILE_SIZE: u64 = 10;
pub fn map_idx(x: u64, y: u64) -> usize {
    ((y * REGION_TILE_WIDTH) + x) as usize
}
