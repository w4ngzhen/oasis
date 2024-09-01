use crate::components::role::Player;
use crate::map::map_builder::MapBuilder;
use crate::resources::CharsetAsset;
use bevy::prelude::*;
use crate::components::MapTile;
use crate::components::position::Position;

pub fn spawn_player(mut commands: Commands, atlas: Res<CharsetAsset>, mut mb: ResMut<MapBuilder>) {
    let player_start = mb.player_init_pos.clone();
    commands.spawn((
        MapTile,
        Player,
        Position { x: player_start.x, y: player_start.y, z: 2 },
        SpriteBundle {
            sprite: Sprite { custom_size: Some(Vec2::new(1.0, 1.0)), ..Default::default() },
            texture: atlas.texture.clone(),
            ..Default::default()
        },
        TextureAtlas { layout: atlas.atlas.clone(), index: '@' as usize },
    ));
}
