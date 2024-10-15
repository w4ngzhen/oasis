use bevy::prelude::*;

#[derive(Component)]
pub struct Attribute {
    /// 力量
    /// 体能的量化
    pub strength: i32,
    /// 敏捷
    /// 灵活度的量化
    pub dexterity: i32,
    /// 韧性
    /// 耐受力的量化
    pub constitution: i32,
    /// 智力
    /// 记忆与思维能力的量化
    pub intelligence: i32,
    /// 感知
    /// 直觉与感受能力的量化
    pub wisdom: i32,
    /// 魅力
    /// 个性气质的量化
    pub charisma: i32,
}

impl Attribute {
    pub fn get_adjusted_attr(attr_value: i32) -> i32 {
        (attr_value - 10) / 2
    }
}
