use crate::components::attack::Attack;
use crate::components::attributes::Attributes;
use crate::components::DestroyObject;
use crate::resources::game_log::GameLog;
use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, ResMut};

pub fn handle_combat(
    mut commands: Commands,
    q_attack: Query<(Entity, &Attack)>,
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
                    commands.spawn(DestroyObject(target_entity));
                }
            }
            if let Some(next_attacker_attr) = next_attacker_attr {
                let (_, mut a_attr) = q_role.get_mut(attacker_entity).unwrap();
                *a_attr = next_attacker_attr;
                if a_attr.current_hp <= 0 {
                    commands.spawn(DestroyObject(attacker_entity));
                }
            }
        }
        commands.entity(attack_msg).despawn_recursive();
    }
}

fn calc_speed(lh: i32, rh: i32) -> bool {
    lh > rh
}
