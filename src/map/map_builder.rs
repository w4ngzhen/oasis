use crate::base::tile_rect::TileRect;
use crate::components::position::Position;
use crate::map::region_map::{RegionMap, TileType};
use crate::prelude::*;
use bevy::prelude::*;
use rand::Rng;

const NUM_ROOMS: usize = 5;

#[derive(Resource)]
pub struct MapBuilder {
    pub region_map: RegionMap,
    walls: Vec<TileRect>,
    rooms: Vec<TileRect>,
}

impl MapBuilder {
    pub fn new() -> Self {
        let mut mb =
            MapBuilder { region_map: RegionMap::new(), rooms: Vec::new(), walls: Vec::new() };

        mb.fill(TileType::Void);
        mb.build_random_rooms();
        mb.build_corridors();
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.region_map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self) {
        let mut rng = rand::thread_rng();

        while self.rooms.len() < NUM_ROOMS {
            let room = TileRect::new(
                rng.gen_range(2..REGION_TILE_WIDTH - 12),
                rng.gen_range(2..REGION_TILE_HEIGHT - 12),
                rng.gen_range(2..12),
                rng.gen_range(2..12),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                let (right, bottom, _) = room.right_bottom().to_tuple();
                let wall = TileRect::new_with_corner(room.x - 1, room.y - 1, right + 1, bottom + 1);
                // First make the floor space that will be the room
                room.for_each(|p| {
                    if p.x > 0 && p.x < REGION_TILE_WIDTH && p.y > 0 && p.y < REGION_TILE_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.region_map.tiles[idx] = TileType::Floor;
                    }
                });
                // now place the walls around it
                wall.for_each(|p| {
                    if p.x > 0 && p.x < REGION_TILE_WIDTH && p.y > 0 && p.y < REGION_TILE_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        if self.region_map.tiles[idx] == TileType::Void {
                            self.region_map.tiles[idx] = TileType::Wall;
                        }
                    }
                });
                self.rooms.push(room);
                self.walls.push(wall);
            }
        }
    }

    fn build_corridors(&mut self) {
        let mut rng = rand::thread_rng();
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.gen_range(0..2) == 1 {
                self.apply_horizontal_tunnel_walls(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel_walls(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel_walls(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel_walls(prev.x, new.x, new.y);
            }
        }
    }

    fn apply_horizontal_tunnel_walls(&mut self, x1: u64, x2: u64, y: u64) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.region_map.try_idx(&Position { x, y, z: 0 }) {
                self.region_map.tiles[idx] = TileType::Floor;
            }
            if let Some(idx) = self.region_map.try_idx(&Position { x, y: y - 1, z: 0 }) {
                if self.region_map.tiles[idx] == TileType::Void {
                    self.region_map.tiles[idx] = TileType::Wall;
                }
            }
            if let Some(idx) = self.region_map.try_idx(&Position { x, y: y + 1, z: 0 }) {
                if self.region_map.tiles[idx] == TileType::Void {
                    self.region_map.tiles[idx] = TileType::Wall;
                }
            }
        }
    }

    fn apply_vertical_tunnel_walls(&mut self, y1: u64, y2: u64, x: u64) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.region_map.try_idx(&Position { x, y, z: 0 }) {
                self.region_map.tiles[idx] = TileType::Floor;
            }
            if let Some(idx) = self.region_map.try_idx(&Position { x: x - 1, y, z: 0 }) {
                if self.region_map.tiles[idx] == TileType::Void {
                    self.region_map.tiles[idx] = TileType::Wall;
                }
            }
            if let Some(idx) = self.region_map.try_idx(&Position { x: x + 1, y, z: 0 }) {
                if self.region_map.tiles[idx] == TileType::Void {
                    self.region_map.tiles[idx] = TileType::Wall;
                }
            }
        }
    }
}

pub fn system_build_map(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let mb = MapBuilder::new();
    commands.insert_resource(mb);
}
