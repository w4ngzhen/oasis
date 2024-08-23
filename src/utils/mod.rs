use crate::constants::TILE_WIDTH;

pub mod rand_gen;

pub fn xy_idx(x: u64, y: u64) -> u64 {
    (y * TILE_WIDTH) + x
}
