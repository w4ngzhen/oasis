use crate::components::position::Position;
use crate::prelude::{REGION_TILE_HEIGHT, REGION_TILE_WIDTH};
use bevy::prelude::Entity;

const REGION_TILE_NUM: u64 = REGION_TILE_WIDTH * REGION_TILE_HEIGHT;

pub struct RegionMap {
    /// 区域某块上存放着一些东西
    pub tiles: Vec<Option<Entity>>,
    // todo 更多的信息
}

impl RegionMap {
    pub fn new() -> Self {
        Self { tiles: vec![None; REGION_TILE_NUM as usize] }
    }

    pub fn in_bounds<TPos: Into<Position>>(pos: TPos) -> bool {
        let position = pos.into();
        position.x >= 0
            && position.x < REGION_TILE_WIDTH
            && position.y >= 0
            && position.y < REGION_TILE_HEIGHT
    }

    pub fn can_enter_tile<T: Into<Position>>(&self, position: T) -> bool {
        let position = position.into();
        // todo check more things.
        self.in_bounds(position)
    }

    pub fn try_idx(&self, position: Position) -> Option<usize> {
        if !self.in_bounds(position) {
            None
        } else {
            Some(map_idx(position.x, position.y))
        }
    }
}

pub fn map_idx(x: u64, y: u64) -> usize {
    ((y * REGION_TILE_WIDTH) + x) as usize
}
