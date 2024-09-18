use bevy::prelude::{Component, Entity};

#[derive(Component, Clone, Copy)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub target: Option<Entity>,
}
