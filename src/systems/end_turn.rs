use crate::game_state::InGamingSubState;
use crate::game_state::InGamingSubState::*;
use bevy::prelude::*;

pub fn end_turn(
    current_state: Res<State<InGamingSubState>>,
    mut next_state: ResMut<NextState<InGamingSubState>>,
) {
    let next: Option<InGamingSubState> = match current_state.get() {
        AwaitingPlayerInput => None,
        PlayerAction => Some(MonsterAction),
        MonsterAction => Some(AwaitingPlayerInput),
        _ => None,
    };

    if let Some(next) = next {
        next_state.set(next);
    }
}
