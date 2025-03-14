use crate::components::map_element::MapElement;
use crate::core_module::game_map::game_map::GameMap;
use crate::core_module::game_map::tile_rect::TileRect;
use crate::core_module::*;
use basic::rand_gen::RandGen;
use bevy::prelude::*;
use rand::Rng;
use std::cmp::{max, min};

const MAX_ROOMS: usize = 10;
const MIN_ROOM_SIZE: i32 = 6;
const MAX_ROOM_SIZE: i32 = 10;

#[derive(Resource)]
pub struct GameMapBuilder {
    pub game_map: GameMap,
    pub rooms: Vec<TileRect>,
}

impl GameMapBuilder {
    pub fn new() -> Self {
        let mut mb =
            GameMapBuilder { game_map: GameMap::new(), rooms: Vec::new() };
        mb.fill(MapElement::Wall);
        mb.build_rooms();
        mb
    }

    fn fill(&mut self, tile: MapElement) {
        self.game_map.elements.iter_mut().for_each(|t| *t = tile);
    }

    fn build_rooms(&mut self) {
        let mut rng = RandGen::new(Some(123));
        let mut generated_rooms: Vec<TileRect> = Vec::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE);
            let h = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE);
            let x = rng.roll_dice(1, GAME_MAP_TILE_WIDTH - w - 1) - 1;
            let y = rng.roll_dice(1, GAME_MAP_TILE_HEIGHT - h - 1) - 1;
            let new_room = TileRect::new(x, y, w, h);
            let mut ok = true;
            for other_room in generated_rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                self.apply_room_to_map(&new_room);
                if !generated_rooms.is_empty() {
                    let (new_x, new_y) = new_room.center().to_tuple();
                    let (prev_x, prev_y) = generated_rooms
                        [generated_rooms.len() - 1]
                        .center()
                        .to_tuple();
                    if rng.range(0, 2) == 1 {
                        self.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        self.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        self.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        self.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                generated_rooms.push(new_room);
            }
        }

        self.rooms = generated_rooms;
    }

    fn apply_room_to_map(&mut self, room: &TileRect) {
        let map_tiles = &mut self.game_map.elements;
        let lt = room.left_top();
        let rb = room.right_bottom();
        for y in lt.y + 1..=rb.y {
            for x in lt.x + 1..=rb.x {
                map_tiles[map_idx(x, y)] = MapElement::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        let map_tiles = &mut self.game_map.elements;
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = map_idx(x, y);
            if idx > 0
                && idx < (GAME_MAP_TILE_WIDTH * GAME_MAP_TILE_HEIGHT) as usize
            {
                map_tiles[idx] = MapElement::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        let map_tiles = &mut self.game_map.elements;
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = map_idx(x, y);
            if idx > 0
                && idx < (GAME_MAP_TILE_WIDTH * GAME_MAP_TILE_HEIGHT) as usize
            {
                map_tiles[idx] = MapElement::Floor;
            }
        }
    }
}
