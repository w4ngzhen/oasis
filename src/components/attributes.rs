use bevy::prelude::*;

#[derive(Component, Copy, Clone, Default)]
pub struct Attributes {
    /// 力量
    /// 决定攻击力，携带物品的最大重量
    pub strength: i32,
    /// 灵巧
    /// 面对攻击的闪避、以及攻击的先后
    pub dexterity: i32,
    /// 韧性
    /// 决定被攻击的防御力
    pub toughness: i32,
    pub max_hp: i32,
    pub current_hp: i32,
    pub max_energy: i32,
    pub current_energy: i32,
    pub damage: i32,
    pub defense: i32,
}

impl Attributes {}
