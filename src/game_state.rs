use bevy::prelude::*;

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
    /// 进入游戏前系统准备
    PrepareGame,
    /// 游戏中 - 玩家回合
    PlayerTurn,
    MonsterTurn,
    /// 暂停游戏
    Paused,
}
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::PlayerTurn)]
pub enum PlayerTurnSubState {
    #[default]
    PlayerAction,
    MapExploring,
    MapPicking,
}
