use crate::components::map_element::MapElement;
use crate::components::position_2d::Position2d;
use crate::core_module::*;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

const NUM_TILES: i32 = GAME_MAP_TILE_WIDTH * GAME_MAP_TILE_HEIGHT;

pub struct GameMap {
    pub elements: Vec<MapElement>,
    /// 存放曾经见过的位置
    pub visited_positions: HashSet<Position2d>
}

impl GameMap {
    pub fn new() -> Self {
        Self {
            elements: vec![MapElement::Void; NUM_TILES as usize],
            visited_positions: HashSet::new(),
        }
    }

    pub fn in_bounds(&self, position: &Position2d) -> bool {
        position.x >= 0
            && position.x < GAME_MAP_TILE_WIDTH
            && position.y >= 0
            && position.y < GAME_MAP_TILE_HEIGHT
    }

    pub fn try_idx(&self, position: &Position2d) -> Option<usize> {
        if !self.in_bounds(position) {
            None
        } else {
            Some(map_idx(position.x, position.y))
        }
    }
}
