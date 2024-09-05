use crate::components::position::Position;
use crate::core_module::*;
use bevy::prelude::*;

const NUM_TILES: i32 = GAME_MAP_TILE_WIDTH * GAME_MAP_TILE_HEIGHT;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    /// 无法到达的地方
    Void,
}

pub struct GameMap {
    /// 地图上的Wall、Floor、Void（虚空）是固定资源，所以不是实体Entity
    pub tiles: Vec<TileType>,
    /// 地图上某些地方占据的东西是实体Entity
    pub occupation: Vec<Option<Entity>>,
}

impl GameMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Void; NUM_TILES as usize],
            occupation: vec![None; NUM_TILES as usize],
        }
    }

    pub fn in_bounds(&self, position: &Position) -> bool {
        position.x >= 0
            && position.x < GAME_MAP_TILE_WIDTH
            && position.y >= 0
            && position.y < GAME_MAP_TILE_HEIGHT
    }

    pub fn can_enter_tile<T: Into<Position>>(&self, position: &Position) -> bool {
        self.in_bounds(position) && (self.tiles[map_idx(position.x, position.y)] == TileType::Floor)
    }

    pub fn is_tile_occupied<T: Into<Position>>(&self, position: &Position) -> bool {
        self.in_bounds(position) && self.occupation[map_idx(position.x, position.y)] != None
    }

    pub fn try_idx(&self, position: &Position) -> Option<usize> {
        if !self.in_bounds(position) {
            None
        } else {
            Some(map_idx(position.x, position.y))
        }
    }
}
