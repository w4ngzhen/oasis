use crate::components::bundles::monster_entity;
use crate::components::item::{Carrier, Item};
use crate::components::name::Naming;
use crate::components::position_2d::Position2d;
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
    let mut monster_idx: usize = 0;
    mb.rooms.iter().skip(1).for_each(|room| {
        let monster_init_pos = room.center();
        let monster_entity = commands.spawn(monster_entity(monster_init_pos, atlas)).id();
        pos_map.insert(monster_init_pos, monster_entity);
        // 给这些monster添加一些物品
        generate_monster_items(&mut commands, monster_idx, monster_entity);
        monster_idx += 1;
    });
    pos_map.iter().for_each(|(pos, entity)| {
        mb.game_map.occupation.insert(*pos, *entity);
    })
}

fn generate_monster_items(cmds: &mut Commands, idx: usize, monster_entity: Entity) -> Entity {
    cmds.spawn((Item::Normal, Naming(format!("monster {idx} item")), Carrier(monster_entity))).id()
}
