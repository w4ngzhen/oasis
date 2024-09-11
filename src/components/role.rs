use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Monster;

#[derive(Bundle, Default)]
pub struct RoleStateBundle {
    pub health: Health,
    pub energy: Energy,
    pub damage: Damage,
    pub defense: Defense,
}

#[derive(Component, Clone, Copy, Default)]
pub struct Health {
    current: i32,
    max: i32,
}

#[derive(Component, Clone, Copy, Default)]
pub struct Energy {
    current: i32,
    max: i32,
}
#[derive(Component, Clone, Copy, Default)]
pub struct Damage {
    current: i32,
}

#[derive(Component, Clone, Copy, Default)]
pub struct Defense {
    current: i32,
}

pub enum RolePart {
    Head,
    Body,
    LeftHand,
    RightHand,
    Leg,
    Foot,
}
