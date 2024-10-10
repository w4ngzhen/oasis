use crate::components::camera::GameMapCamera;
use crate::game_state::InGamingSubState;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::utils::default;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};

/// UI 系统，需要在Update中运行
pub fn game_ui_system(
    mut q_map_camera: Query<&mut Camera, With<GameMapCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut egui_ctx: EguiContexts,
    current_state: Option<Res<State<InGamingSubState>>>, // 注意，要使用Option来处理
    mut next_state: ResMut<NextState<InGamingSubState>>,
) {
    let ctx = egui_ctx.ctx_mut();
    let mut camera = q_map_camera.get_single_mut().expect("No camera found");
    let window = q_window.get_single().expect("No primary window found");

    let left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Left resizeable panel");
            ui.allocate_rect(
                ui.available_rect_before_wrap(),
                egui::Sense::hover(),
            );
        })
        .response
        .rect
        .width();
    let right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            ui.allocate_rect(
                ui.available_rect_before_wrap(),
                egui::Sense::hover(),
            );
        })
        .response
        .rect
        .width();
    let top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Top resizeable panel");
            ui.allocate_rect(
                ui.available_rect_before_wrap(),
                egui::Sense::hover(),
            );
        })
        .response
        .rect
        .height();
    let bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Bottom resizeable panel");
            ui.allocate_rect(
                ui.available_rect_before_wrap(),
                egui::Sense::hover(),
            );
        })
        .response
        .rect
        .height();

    let pos = UVec2::new(left as u32, top as u32);
    let size = UVec2::new(window.physical_width(), window.physical_height())
        - pos
        - UVec2::new(right as u32, bottom as u32);

    camera.viewport = Some(Viewport {
        physical_position: pos,
        physical_size: size,
        ..default()
    });
    if let Some(current_state) = current_state {
        match current_state.get() {
            InGamingSubState::Modal => {
                egui::Window::new("Modal")
                    .min_size([800.0, 600.0])
                    .default_size([800.0, 600.0])
                    .default_pos([80.0, 80.0])
                    .constrain(true)
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Modal");
                        if ui.button("Ok").clicked() {
                            next_state
                                .set(InGamingSubState::AwaitingPlayerInput);
                        }
                    });
            }
            _ => {}
        }
    }
}
