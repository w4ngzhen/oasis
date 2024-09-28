use crate::components::map_tile::MapElementType;
use bevy::prelude::*;

pub struct TileRenderDescriptor {
    // the index in the atlas sprite sheet
    pub tile_index: usize,
    // the color to tint the glyph
    pub color: Color,
}

impl TileRenderDescriptor {
    pub fn new(tile_index: usize, color: Color) -> Self {
        Self { tile_index, color }
    }
}

pub fn tile_to_render_descriptor(
    map_ele_type: MapElementType,
) -> Option<TileRenderDescriptor> {
    let wall_color = Color::srgba(1., 1., 1., 1.0);
    let floor_color = Color::srgba(0.3, 0.3, 0.3, 1.0);

    match map_ele_type {
        // index = 7 is a point
        MapElementType::Floor => {
            Some(TileRenderDescriptor::new(7, floor_color))
        }
        MapElementType::Wall => {
            Some(TileRenderDescriptor::new('#' as usize, wall_color))
        }
        _ => None,
    }
}
