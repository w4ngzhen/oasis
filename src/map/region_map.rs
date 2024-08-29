use crate::components::position::Position;
use crate::components::MapTile;
use crate::map::map_builder::MapBuilder;
use crate::map::themes::tile_to_render_descriptor;
use crate::prelude::*;
use crate::resources::CharsetAsset;
use bevy::prelude::*;

const NUM_TILES: u64 = REGION_TILE_WIDTH * REGION_TILE_HEIGHT;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    /// 无法到达的地方
    Void,
}

pub struct RegionMap {
    /// 地图上的Wall、Floor、Void（虚空）是固定资源，所以不是实体Entity
    pub tiles: Vec<TileType>,
    /// 地图上某些地方占据的东西是实体Entity
    pub occupation: Vec<Option<Entity>>,
}

impl RegionMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Void; NUM_TILES as usize],
            occupation: vec![None; NUM_TILES as usize],
        }
    }

    pub fn in_bounds(&self, position: &Position) -> bool {
        position.x >= 0
            && position.x < REGION_TILE_WIDTH
            && position.y >= 0
            && position.y < REGION_TILE_HEIGHT
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

pub fn spawn_map_tiles(
    mut commands: Commands,
    mb: Res<MapBuilder>,
    charset_asset: Res<CharsetAsset>,
) {
    for y in 0..REGION_TILE_HEIGHT {
        for x in 0..REGION_TILE_WIDTH {
            let idx = map_idx(x, y);
            let render_descriptor = tile_to_render_descriptor(mb.region_map.tiles[idx]);

            if let Some(render_descriptor) = render_descriptor {
                match mb.region_map.tiles[idx] {
                    TileType::Floor => {
                        commands.spawn((
                            MapTile,
                            Position { x, y, z: 1 },
                            SpriteBundle {
                                sprite: Sprite {
                                    color: render_descriptor.color,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            TextureAtlas {
                                layout: charset_asset.atlas.clone(),
                                index: render_descriptor.tile_index,
                            },
                        ));
                    }
                    TileType::Wall => {
                        if let Some(bkg_color) = render_descriptor.bg_color {
                            // 背景色
                            commands.spawn((
                                MapTile,
                                Position { x, y, z: 0 }, // z = 0, background color.
                                SpriteBundle {
                                    sprite: Sprite { color: bkg_color, ..Default::default() },
                                    ..Default::default()
                                },
                            ));
                        }

                        commands.spawn((
                            MapTile,
                            Position { x, y, z: 1 }, // z = 1, foreground color.
                            SpriteBundle {
                                sprite: Sprite {
                                    color: render_descriptor.color,
                                    ..Default::default()
                                },
                                texture: charset_asset.texture.clone(),
                                ..Default::default()
                            },
                            TextureAtlas {
                                layout: charset_asset.atlas.clone(),
                                index: render_descriptor.tile_index,
                            },
                        ));
                    }
                    TileType::Void => (),
                }
            }
        }
    }
}
