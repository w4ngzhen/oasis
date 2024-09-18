use crate::components::field_of_vision::FieldOfVision;
use crate::components::position_2d::Position2d;
use crate::components::role::{Monster, Player};
use crate::components::WantsToMove;
use crate::core_module::game_map::game_map::GameMap;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::game_state::InGamingSubState;
use bevy::prelude::*;
use pathfinding::prelude::astar;

pub fn monster_chasing(
    mut commands: Commands,
    q_monster: Query<(Entity, &Position2d, &FieldOfVision), With<Monster>>,
    q_player: Query<&Position2d, With<Player>>,
    mb: Res<GameMapBuilder>,
) {
    let player_pos = q_player.single();
    for (monster_entity, monster_pos, fov) in q_monster.iter() {
        if fov.visible_tiles.contains(player_pos) {
            info!("monster saw you!");
            // calc path
            let path = calc_path(monster_pos, player_pos, &mb.game_map);
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
fn calc_path(src: &Position2d, dest: &Position2d, map: &GameMap) -> Vec<Position2d> {
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos(i32, i32);
    impl Pos {
        fn successors(&self, map: &GameMap, dest_pos: Self) -> Vec<(Pos, u64)> {
            let &Pos(curr_x, curr_y) = self;
            // 从当前点开始，下一次能走四个方位：上下左右
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
                    // 2. 或没有占据的点
                    *p == dest_pos || !map.is_occupied(&Position2d::new(p.0, p.1))
                })
                .map(|p| (p, 1))
                .collect();
            // info!("successors: {:?}", next);
            next
        }
    }

    let src_pos = Pos(src.x, src.y);
    let dest_pos = Pos(dest.x, dest.y);
    let result = astar(
        &src_pos,
        |p| p.successors(map, dest_pos.clone()),
        |p| (p.0.abs_diff(dest_pos.0) + p.1.abs_diff(dest_pos.1)) as u64,
        |p| *p == dest_pos,
    );
    if result == None {
        return vec![];
    }
    let res = result.unwrap();
    res.0.iter().map(|item| Position2d { x: item.0, y: item.1 }).collect()
}
