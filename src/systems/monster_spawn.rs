use crate::components::attributes::Attributes;
use crate::components::field_of_vision::FieldOfVision;
use crate::components::position_2d::{Position2d, PositionZIndex};
use crate::components::role::Monster;
use crate::components::MapTileElement;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::resources::CharsetAsset;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub fn spawn_monster(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mut mb: ResMut<GameMapBuilder>,
) {
    let mut pos_map: HashMap<Position2d, Entity> = HashMap::new();
    mb.rooms.iter().skip(1).for_each(|room| {
        let monster_init_pos = room.center();
        let monster_entity = commands
            .spawn((
                MapTileElement { color: Color::srgba(0., 1., 0., 1.), is_background: false },
                Monster,
                Attributes {
                    strength: 0,
                    dexterity: 0,
                    toughness: 0,
                    max_hp: 20,
                    current_hp: 20,
                    max_energy: 0,
                    current_energy: 0,
                    damage: 15,
                    defense: 10,
                },
                FieldOfVision { visible_tiles: Vec::new(), range: 8, is_dirty_data: true },
                Position2d { x: monster_init_pos.x, y: monster_init_pos.y },
                PositionZIndex(2),
                SpriteBundle {
                    sprite: Sprite { custom_size: Some(Vec2::new(1.0, 1.0)), ..Default::default() },
                    texture: atlas.texture.clone(),
                    ..Default::default()
                },
                TextureAtlas { layout: atlas.atlas.clone(), index: 'g' as usize },
            ))
            .id();
        pos_map.insert(monster_init_pos, monster_entity);
    });
    pos_map.iter().for_each(|(pos, entity)| {
        mb.game_map.occupation.insert(*pos, *entity);
    })
}
