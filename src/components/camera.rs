use bevy::prelude::*;

/// 游戏地图摄像机，只关注地图场景渲染
#[derive(Component, Clone, Copy)]
pub struct GameMapCamera;

/// 游戏HUD模块Camera
#[derive(Component, Clone, Copy)]
pub struct GameHudCamera;
