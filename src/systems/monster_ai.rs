use crate::components::field_of_vision::FieldOfVision;
use crate::components::position_2d::Position2d;
use crate::components::role::{Monster, Player};
use crate::components::Movement;
use crate::core_module::game_map::game_map::GameMap;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
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
            // calc path
            let path = calc_path(monster_pos, player_pos, &mb.game_map);
            if let Some(first_pos) = path.first() {
                commands.spawn(Movement {
                    entity: monster_entity.clone(),
                    destination: first_pos.clone(),
                });
            }
        }
    }
}

/// calc path by a-star
fn calc_path(src: &Position2d, dest: &Position2d, map: &GameMap) -> Vec<Position2d> {
    const SCALE: u64 = 10;
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos(i32, i32);
    impl Pos {
        fn successors(&self, map: &GameMap) -> Vec<(Pos, u64)> {
            let &Pos(curr_x, curr_y) = self;
            // 从当前点开始，下一次能走四个方位
            let next: Vec<(Pos, u64)> = vec![
                Pos(curr_x, curr_y - 1),
                Pos(curr_x - 1, curr_y),
                Pos(curr_x + 1, curr_y),
                Pos(curr_x, curr_y + 1),
            ]
            .into_iter()
            .filter(|p| {
                // 过滤掉不能到达的点
                map.is_occupied(&Position2d::new(p.0, p.1))
            })
            .map(|p| (p, 1))
            .collect();
            next
        }
    }

    let src_pos = Pos(src.x, src.y);
    let dest_pos = Pos(dest.x, dest.y);
    let result = astar(
        &src_pos,
        |p| p.successors(map),
        |p| (p.0.abs_diff(dest_pos.0) + p.1.abs_diff(dest_pos.1)) as u64,
        |p| *p == dest_pos,
    );
    if result == None {
        return vec![];
    }
    let res = result.unwrap();
    res.0.iter().map(|item| Position2d { x: item.0, y: item.1 }).collect()
}
