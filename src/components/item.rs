use crate::components::role::RolePart;
use bevy::prelude::{Component, Entity};

#[derive(Component, Default)]
pub enum Item {
    #[default]
    Normal,
    Weapon(RolePart),
    Medicine,
    Scroll,
    Container(Vec<Entity>),
}

#[derive(Component, Copy, Clone)]
pub struct ItemId(pub i32);

pub const CONTAINER_ITEM_ID: ItemId = ItemId(0x0999);

/// 被谁携带
#[derive(Component)]
pub struct Carrier(pub Entity);

/// 被谁装备
#[derive(Component)]
pub struct Equipped(pub Entity);
