use crate::core_module::game_map::game_map::TileType;
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
    fn new(tile_index: usize, color: Color, bg_color: Option<Color>) -> Self {
        Self { tile_index, color, bg_color }
    }
}

pub fn tile_to_render_descriptor(tile_type: TileType) -> Option<TileRenderDescriptor> {
    let wall_color = Color::srgba(1., 1., 1., 1.0);
    let floor_color = Color::srgba(0.3, 0.3, 0.3, 1.0);
    let bg_color = Color::srgba(0., 0., 0., 1.);

    match tile_type {
        // index = 7 is a point
        TileType::Floor => Some(TileRenderDescriptor::new(7, floor_color, Some(bg_color))),
        TileType::Wall => Some(TileRenderDescriptor::new('#' as usize, wall_color, Some(bg_color))),
        _ => None,
    }
}
