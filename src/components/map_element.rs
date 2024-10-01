use bevy::prelude::*;

/// 地图元素
#[derive(Component, Copy, Clone, PartialOrd, PartialEq)]
pub enum MapElement {
    Void,
    Role,
    Wall,
    Floor,
    MapItem,
    SystemItem,
    Custom(i32),
}

/// 地图上元素的属性
#[derive(Component, Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct MapElementProp {
    /// 是否遮挡视野，决定了视野能否穿透
    pub is_block_view: bool,
    /// 是否碰撞体，决定了能否和其他的元素重叠
    pub is_collision: bool,
}

impl MapElementProp {
    /// 一些物体的基本属性
    pub fn get(ele: MapElement) -> Self {
        match ele {
            MapElement::Void => {
                Self { is_collision: false, is_block_view: true }
            }
            MapElement::Wall => {
                Self { is_collision: true, is_block_view: true }
            }
            MapElement::Floor => {
                Self { is_collision: false, is_block_view: false }
            }
            _ => Self { is_collision: false, is_block_view: true },
        }
    }
}

/// 匹配MapElement::SystemItem
#[derive(Component)]
pub struct SystemItemPickCursor;