use crate::components::map_element::MapElement;
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
    map_ele: MapElement,
) -> Option<TileRenderDescriptor> {
    match map_ele {
        // index = 7 is a point
        MapElement::Floor => {
            Some(TileRenderDescriptor::new(7, Color::srgba(1., 1., 1., 1.0)))
        }
        MapElement::Wall => Some(TileRenderDescriptor::new(
            '#' as usize,
            Color::srgba(0.3, 0.3, 0.3, 1.0),
        )),
        _ => None,
    }
}
