use crate::components::role::RolePart;
use bevy::prelude::{Component, Entity};

#[derive(Component, Default)]
pub struct ItemCollection {
    items: Vec<Entity>,
}

#[derive(Component, Default)]
pub enum Item {
    #[default]
    Normal,
    Weapon(RolePart),
    Medicine,
    Scroll,
}

/// 被谁携带
#[derive(Component)]
pub struct Carrier(pub Entity);

/// 被谁装备
#[derive(Component)]
pub struct Equipped(pub Entity);
