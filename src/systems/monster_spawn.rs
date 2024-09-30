use crate::components::bundles::monster_bundle;
use crate::components::item::{Carrier, Equipped, Item, ItemId};
use crate::components::position_2d::Position2d;
use crate::components::role::RolePart;
use crate::components::Naming;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::resources::CharsetAsset;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub fn spawn_monster(
    mut commands: Commands,
    charset_asset: Res<CharsetAsset>,
    mut mb: ResMut<GameMapBuilder>,
) {
    let mut pos_map: HashMap<Position2d, Entity> = HashMap::new();
    let mut monster_idx: usize = 0;
    mb.rooms.iter().skip(1).for_each(|room| {
        let monster_init_pos = room.center();
        let monster_name = Naming(format!("monster#{monster_idx}"));
        let monster_entity = commands
            .spawn(monster_bundle(monster_init_pos, &charset_asset))
            .insert(monster_name.clone())
            .id();
        pos_map.insert(monster_init_pos, monster_entity);
        // 给这些monster添加一些物品
        if monster_idx % 2 != 0 {
            generate_monster_some_items(&mut commands, monster_entity);
        } else {
            generate_monster_single_item(&mut commands, monster_entity);
        }
        info!("generate {}", monster_name.0);
        monster_idx += 1;
    });
}
/// 给怪物添加装备以及携带的物品
fn generate_monster_some_items(cmds: &mut Commands, monster_entity: Entity) {
    let monster_name = format!("monster@{monster_entity}");
    cmds.spawn((
        ItemId(0),
        Item::Normal,
        Naming(format!("{monster_name}-Item")),
        Carrier(monster_entity),
    ));
    cmds.spawn((
        ItemId(0x01),
        Item::Weapon(RolePart::Body),
        Naming(format!("{monster_name}-Armor")),
        Equipped(monster_entity),
    ));
    cmds.spawn((
        ItemId(0x02),
        Item::Weapon(RolePart::RightHand),
        Naming(format!("{monster_name}-RightHand Knife")),
        Equipped(monster_entity),
    ));
}

fn generate_monster_single_item(cmds: &mut Commands, monster_entity: Entity) {
    let monster_name = format!("monster@{monster_entity}");
    cmds.spawn((
        ItemId(0),
        Item::Normal,
        Naming(format!("{monster_name}-Item")),
        Carrier(monster_entity),
    ));
}
