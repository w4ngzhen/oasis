use crate::components::attack::WantsToAttack;
use crate::components::attributes::Attributes;
use crate::components::item::{Carrier, Equipped, Item};
use crate::components::position_2d::{Position2d, PositionZIndex};
use crate::components::{MapTileElement, Naming, WantsToDestroy};
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::resources::game_log::GameLog;
use crate::resources::CharsetAsset;
use bevy::log::info;
use bevy::prelude::*;
use bevy::reflect::List;

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
    q_position: Query<&Position2d>,
    q_destroy: Query<(Entity, &WantsToDestroy)>,
    q_carrier: Query<(Entity, &Carrier), With<Item>>,
    q_name: Query<&Naming, With<Item>>,
    atlas: Res<CharsetAsset>,
    // q_equipped_item: Query<(Entity, &Equipped), With<Item>>,
) {
    for (msg, destroy) in q_destroy.iter() {
        let be_destroyed_entity = destroy.0;
        let be_destroyed_entity_pos =
            q_position.get(be_destroyed_entity).unwrap();
        // if it has items, place theme at floor
        let carried_entities = q_carrier
            .iter()
            .filter_map(|(item_entity, carrier)| {
                if carrier.0 == be_destroyed_entity {
                    Some(item_entity)
                } else {
                    None
                }
            })
            .collect::<Vec<Entity>>();
        let item_len = carried_entities.len();
        if item_len == 1 {
            let item = carried_entities[0];
            commands.entity(item).remove::<Carrier>().insert((
                MapTileElement {
                    color: Default::default(),
                    is_background: false,
                },
                be_destroyed_entity_pos.clone(),
                PositionZIndex(2),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..Default::default()
                    },
                    texture: atlas.texture.clone(),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: atlas.atlas.clone(),
                    index: '@' as usize,
                },
            ));
        }
        commands.entity(be_destroyed_entity).despawn_recursive();
        mb.game_map.remove_entity(be_destroyed_entity);
        info!("{:?} died.", be_destroyed_entity);
        commands.entity(msg).despawn_recursive();
    }
}
