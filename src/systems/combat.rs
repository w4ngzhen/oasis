use crate::components::attributes::Attributes;
use crate::components::bundles::element_render_bundle;
use crate::components::item::{Carrier, Equipped, Item, CONTAINER_ITEM_ID};
use crate::components::map_element::{MapElement, MapElementProp};
use crate::components::msg::{WantsToAttack, WantsToDestroy};
use crate::components::position_2d::{Position2d, PositionZIndex};
use crate::components::{MapTileElement, Naming};
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::core_module::GAME_MAP_TILE_WIDTH;
use crate::resources::game_log::GameLog;
use crate::resources::CharsetAsset;
use crate::utils::get_charset_index;
use bevy::log::info;
use bevy::prelude::*;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;

pub fn handle_combat(
    mut commands: Commands,
    q_attack: Query<(Entity, &WantsToAttack)>,
    mut q_role: Query<(Entity, &mut Attributes)>,
    _game_log: ResMut<GameLog>,
) {
    for (attack_msg, attack) in q_attack.iter() {
        let attacker_entity = attack.attacker;
        let target = attack.target;
        if let Some(target_entity) = target {
            let mut next_target_attr: Option<Attributes> = None;
            let mut next_attacker_attr: Option<Attributes> = None;
            if let (Ok((_, a_attr)), Ok((_, t_attr))) =
                (q_role.get(attacker_entity), q_role.get(target_entity))
            {
                let to_damage = (a_attr.damage - t_attr.defense).max(0);
                if to_damage > 0 {
                    let mut attr = t_attr.clone();
                    attr.current_hp = (t_attr.current_hp - to_damage).max(0);
                    next_target_attr = Some(attr);
                }
                let be_damaged = (t_attr.damage - a_attr.defense).max(0);
                if be_damaged > 0 {
                    let mut attr = a_attr.clone();
                    attr.current_hp = (a_attr.current_hp - be_damaged).max(0);
                    next_attacker_attr = Some(attr);
                }
            }

            if let Some(next_target_attr) = next_target_attr {
                let (_, mut t_attr) = q_role.get_mut(target_entity).unwrap();
                *t_attr = next_target_attr;
                if t_attr.current_hp <= 0 {
                    commands.spawn(WantsToDestroy(target_entity));
                }
            }
            if let Some(next_attacker_attr) = next_attacker_attr {
                let (_, mut a_attr) = q_role.get_mut(attacker_entity).unwrap();
                *a_attr = next_attacker_attr;
                if a_attr.current_hp <= 0 {
                    commands.spawn(WantsToDestroy(attacker_entity));
                }
            }
        }
        commands.entity(attack_msg).despawn_recursive();
    }
}

pub fn handle_object_destroy(
    mut commands: Commands,
    mut mb: ResMut<GameMapBuilder>,
    charset_asset: Res<CharsetAsset>,
    q_position: Query<&Position2d>,
    q_destroy: Query<(Entity, &WantsToDestroy)>,
    q_carrier: Query<(Entity, &Carrier), With<Item>>,
    q_equipped: Query<(Entity, &Equipped), With<Item>>,
    // q_name: Query<&Naming, With<Item>>,
) {
    for (msg, destroy) in q_destroy.iter() {
        let be_destroyed_entity = destroy.0;

        info!("{:?} died.", be_destroyed_entity);
        // 爆装备、爆物品
        let be_destroyed_entity_pos =
            q_position.get(be_destroyed_entity).unwrap();
        // 携带的物品
        let carried_items = get_carried_items(be_destroyed_entity, &q_carrier);
        // 装备的物品
        let equipped_items =
            get_equipped_items(be_destroyed_entity, &q_equipped);
        let total_item_len = carried_items.len() + equipped_items.len();
        let mut all_items: Vec<Entity> = Vec::with_capacity(total_item_len);
        for item in carried_items {
            all_items.push(item);
        }
        for item in equipped_items {
            all_items.push(item);
        }
        let item_pos = be_destroyed_entity_pos.clone();
        let generated_item_info = match total_item_len {
            0 => {
                // nothing.
                None
            }
            1 => {
                let single_item = all_items.first().unwrap().clone();
                commands.entity(single_item).insert((
                    MapElement::MapItem,
                    MapElementProp {
                        // 暂时这样设计
                        is_collision: false,
                        is_block_view: false,
                    },
                    MapTileElement {
                        color: Color::srgb_u8(244, 187, 120),
                        ..default()
                    },
                    item_pos.clone(), // 将物品放置到对应位置
                    PositionZIndex(2),
                    element_render_bundle(
                        get_charset_index(1, 13),
                        &charset_asset,
                    ),
                ));
                // only one.
                Some(single_item)
            }
            _ => {
                // over one.
                // generate a container
                let item_collection = commands
                    .spawn((
                        CONTAINER_ITEM_ID,
                        MapElement::MapItem,
                        MapElementProp {
                            // 暂时这样设计
                            is_collision: false,
                            is_block_view: false,
                        },
                        Item::Container(all_items),
                        MapTileElement {
                            color: Color::srgb_u8(244, 187, 120),
                            ..default()
                        },
                        item_pos.clone(), // 将物品放置到对应位置
                        PositionZIndex(2),
                        Naming("some items.".into()),
                        element_render_bundle(
                            get_charset_index(14, 10),
                            &charset_asset,
                        ),
                    ))
                    .id();
                Some(item_collection)
            }
        };
        // 清理掉这个被摧毁的实体
        commands.entity(be_destroyed_entity).despawn_recursive();
        // 清理消息
        commands.entity(msg).despawn_recursive();
    }
}

fn get_carried_items(
    target: Entity,
    q_carrier: &Query<(Entity, &Carrier), With<Item>>,
) -> Vec<Entity> {
    q_carrier
        .iter()
        .filter_map(
            |(entity, carrier)| {
                if carrier.0 == target {
                    Some(entity)
                } else {
                    None
                }
            },
        )
        .collect::<Vec<Entity>>()
}

fn get_equipped_items(
    target: Entity,
    q_equipped: &Query<(Entity, &Equipped), With<Item>>,
) -> Vec<Entity> {
    q_equipped
        .iter()
        .filter_map(
            |(entity, equipped)| {
                if equipped.0 == target {
                    Some(entity)
                } else {
                    None
                }
            },
        )
        .collect::<Vec<Entity>>()
}
