use crate::components::map_tile::MapElementType;
use crate::components::position_2d::Position2d;
use crate::core_module::*;
use bevy::prelude::*;
use std::collections::HashMap;

const NUM_TILES: i32 = GAME_MAP_TILE_WIDTH * GAME_MAP_TILE_HEIGHT;

pub struct GameMap {
    /// 地图上的Wall、Floor、Void（虚空）是固定资源，所以不是实体Entity
    pub tiles: Vec<MapElementType>,
    /// 见过的tile
    pub visited_tiles: Vec<bool>,
    /// 地图上某些地方占据的东西是实体Entity
    /// 包括：玩家、怪物、物品等等
    pub occupation: HashMap<Position2d, Entity>,
}

impl GameMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![MapElementType::Void; NUM_TILES as usize],
            visited_tiles: vec![false; NUM_TILES as usize],
            occupation: HashMap::new(),
        }
    }

    pub fn in_bounds(&self, position: &Position2d) -> bool {
        position.x >= 0
            && position.x < GAME_MAP_TILE_WIDTH
            && position.y >= 0
            && position.y < GAME_MAP_TILE_HEIGHT
    }

    pub fn can_enter_tile<T: Into<Position2d>>(
        &self,
        position: &Position2d,
    ) -> bool {
        self.in_bounds(position)
            && (self.tiles[map_idx(position.x, position.y)]
                == MapElementType::Floor)
    }

    pub fn is_tile_opacity(&self, position: &Position2d) -> bool {
        self.in_bounds(position)
            && self.tiles[map_idx(position.x, position.y)]
                == MapElementType::Wall
    }

    pub fn is_occupied(&self, position: &Position2d) -> bool {
        self.in_bounds(position) && self.occupation.contains_key(&position)
    }

    pub fn remove_entity(&mut self, target_entity: Entity) {
        let mut target_pos: Option<Position2d> = None;
        for (pos, entity) in self.occupation.iter_mut() {
            if *entity == target_entity {
                target_pos = Some(pos.clone());
            }
        }
        if let Some(pos) = target_pos {
            self.occupation.remove(&pos);
        }
    }

    pub fn move_entity(&mut self, target_entity: Entity, next_pos: Position2d) {
        let mut target_pos: Option<Position2d> = None;
        for (pos, entity) in self.occupation.iter() {
            if *entity == target_entity {
                target_pos = Some(pos.clone());
            }
        }
        if let Some(pos) = target_pos {
            if pos != next_pos {
                self.occupation.remove(&pos);
                self.occupation.insert(next_pos, target_entity);
            }
        }
    }

    pub fn try_idx(&self, position: &Position2d) -> Option<usize> {
        if !self.in_bounds(position) {
            None
        } else {
            Some(map_idx(position.x, position.y))
        }
    }

    pub fn print_occupations(&self) {
        let mut list = self.occupation.keys().collect::<Vec<&Position2d>>();
        list.sort_by(|p1, p2| {
            return p1.to_index().cmp(&p2.to_index());
        });
        info!("{:?}", list);
    }
}
