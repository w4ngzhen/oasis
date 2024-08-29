use crate::map::region_map::TileType;
use bevy::prelude::*;

pub struct TileRenderDescriptor {
    // the index in the atlas sprite sheet
    pub tile_index: usize,
    // the color to tint the glyph
    pub color: Color,
    // the background color. If the glyph uses the full cell, not needed
    pub bg_color: Option<Color>,
}

impl TileRenderDescriptor {
    fn new(tile_index: usize, color: Color, bg_color: Color) -> Self {
        Self { tile_index, color, bg_color: Some(bg_color) }
    }

    fn new_with_no_bg_color(tile_index: usize, color: Color) -> Self {
        Self { tile_index, color, bg_color: None }
    }
}

pub fn tile_to_render_descriptor(tile_type: TileType) -> Option<TileRenderDescriptor> {
    let glyph_color = Color::srgba(1., 0.3, 0.3, 1.0);
    let wall_color = Color::srgba(1., 0.05, 0.05, 1.0);
    let floor_color = Color::srgba(0.529, 0.529, 0.529, 1.0);

    match tile_type {
        // index = 219 is a full square
        TileType::Floor => Some(TileRenderDescriptor::new_with_no_bg_color(3, floor_color)),
        TileType::Wall => Some(TileRenderDescriptor::new('#' as usize, glyph_color, wall_color)),
        _ => None,
    }
}
