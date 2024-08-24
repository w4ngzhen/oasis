use bevy::prelude::States;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    /// 入口
    InPortal,
    /// 游戏主菜单
    InMainMenu,
    /// 玩家配置（角色、场景...）
    InPlayerConfig,
    /// 游戏配置
    InGameSetting,
    /// 游戏中
    InGaming,
    /// 暂停游戏
    Paused,
}
