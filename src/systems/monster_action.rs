use crate::components::field_of_vision::FieldOfVision;
use crate::components::map_element::{MapElement, MapElementProp};
use crate::components::msg::WantsToMove;
use crate::components::position_2d::Position2d;
use crate::components::role::{Monster, Player};
use crate::core_module::game_map::game_map::GameMap;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::game_state::InGamingSubState;
use bevy::prelude::*;
use pathfinding::prelude::astar;

pub fn monster_chasing(
    mut commands: Commands,
    q_monster: Query<(Entity, &Position2d, &FieldOfVision), With<Monster>>,
    q_player: Query<&Position2d, With<Player>>,
    q_map_ele_prop: Query<(&Position2d, &MapElementProp)>,
) {
    let player_pos = q_player.single();
    for (monster_entity, monster_pos, fov) in q_monster.iter() {
        if fov.visible_positions.contains(player_pos) {
            info!("monster saw you!");
            // calc path
            let path = calc_path(monster_pos, player_pos, &q_map_ele_prop);
            info!("path: {:?}", path);
            // path的第一个就是自己的位置，所以要跳过
            if let Some(first_pos) = path.iter().skip(1).next() {
                info!("monster will move to you. {:?}", first_pos);
                commands.spawn(WantsToMove {
                    entity: monster_entity.clone(),
                    destination: first_pos.clone(),
                });
            }
        }
    }
}

/// calc path by a-star
/// 计算src到dest的，基于A星算法的路径查找
fn calc_path(
    src: &Position2d,
    dest: &Position2d,
    q_map_ele_prop: &Query<(&Position2d, &MapElementProp)>,
) -> Vec<Position2d> {
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos(i32, i32);
    fn successors(
        curr_pos: &Pos,
        dest_pos: &Pos,
        q_map_ele_prop: &Query<(&Position2d, &MapElementProp)>,
    ) -> Vec<(Pos, u64)> {
        // 从当前点开始，下一次能走四个方位：上下左右
        let curr_x = curr_pos.0;
        let curr_y = curr_pos.1;
        let next: Vec<(Pos, u64)> = vec![
            Pos(curr_x, curr_y - 1),
            Pos(curr_x, curr_y + 1),
            Pos(curr_x - 1, curr_y),
            Pos(curr_x + 1, curr_y),
        ]
        .into_iter()
        .filter(|p| {
            // 保留可到达的点：
            // 1. 就是终点
            if *p == *dest_pos {
                return true;
            }
            // 2. 或没有碰撞体的点
            let has_collision = q_map_ele_prop
                .iter()
                .filter_map(|(map_pos, prop)| {
                    if map_pos.x == p.0 && map_pos.y == p.1 {
                        Some(prop.is_collision)
                    } else {
                        None
                    }
                })
                .any(|is_collision| is_collision);
            !has_collision
        })
        .map(|p| (p, 1))
        .collect();
        next
    }

    let src_pos = Pos(src.x, src.y);
    let dest_pos = Pos(dest.x, dest.y);
    let result = astar(
        &src_pos,
        |p| successors(p, &dest_pos, q_map_ele_prop),
        |p| (p.0.abs_diff(dest_pos.0) + p.1.abs_diff(dest_pos.1)) as u64,
        |p| *p == dest_pos,
    );
    if result == None {
        return vec![];
    }
    let res = result.unwrap();
    res.0.iter().map(|item| Position2d { x: item.0, y: item.1 }).collect()
}
