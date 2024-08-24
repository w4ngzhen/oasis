mod components;
mod resources;
mod systems;
use crate::game_state::GameState;
use crate::in_portal::components::OnPortalScreen;
use crate::in_portal::systems::{countdown, setup_portal};
use crate::utils::destroy_entity;
use bevy::prelude::*;

pub struct InPortalPlugin;

impl Plugin for InPortalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InPortal), setup_portal);
        app.add_systems(OnExit(GameState::InPortal), destroy_entity::<OnPortalScreen>);
        app.add_systems(Update, countdown.run_if(in_state(GameState::InPortal)));
    }
}
