use bevy::prelude::{Component, Entity};

#[derive(Component, Clone, Copy)]
pub struct Attack {
    pub attacker: Entity,
    pub target: Option<Entity>,
}
