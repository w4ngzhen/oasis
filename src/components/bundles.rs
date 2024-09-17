use crate::components::attributes::Attributes;
use crate::components::field_of_vision::FieldOfVision;
use crate::components::position_2d::{Position2d, PositionZIndex};
use crate::components::role::{Monster, Player};
use crate::components::MapTileElement;
use crate::resources::CharsetAsset;
use bevy::prelude::*;

pub fn player_entity(player_init_pos: Position2d, atlas: Res<CharsetAsset>) -> impl Bundle {
    (
        Player,
        MapTileElement { color: Color::WHITE, is_background: false },
        Attributes { max_hp: 100, current_hp: 100, damage: 20, defense: 10, ..default() },
        FieldOfVision { visible_tiles: Vec::new(), range: 8, is_dirty_data: true },
        Position2d { x: player_init_pos.x, y: player_init_pos.y },
        PositionZIndex(2),
        SpriteBundle {
            sprite: Sprite { custom_size: Some(Vec2::new(1.0, 1.0)), ..Default::default() },
            texture: atlas.texture.clone(),
            ..Default::default()
        },
        TextureAtlas { layout: atlas.atlas.clone(), index: '@' as usize },
    )
}

pub fn monster_entity(monster_init_pos: Position2d, atlas: Res<CharsetAsset>) -> impl Bundle {
    (
        MapTileElement { color: Color::srgba(0., 1., 0., 1.), is_background: false },
        Monster,
        Attributes { max_hp: 20, current_hp: 20, damage: 15, defense: 10, ..default() },
        FieldOfVision { visible_tiles: Vec::new(), range: 8, is_dirty_data: true },
        Position2d { x: monster_init_pos.x, y: monster_init_pos.y },
        PositionZIndex(2),
        SpriteBundle {
            sprite: Sprite { custom_size: Some(Vec2::new(1.0, 1.0)), ..Default::default() },
            texture: atlas.texture.clone(),
            ..Default::default()
        },
        TextureAtlas { layout: atlas.atlas.clone(), index: 'g' as usize },
    )
}
