use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};
use bevy_egui::{egui, EguiContexts};

use crate::Player;

pub fn debug_ui(
    mut contexts: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
    mut query_player: Query<&mut GlobalTransform, With<Player>>,
) {
    let transform_player = query_player.single_mut();
    egui::SidePanel::left("DebugUI")
        .resizable(false)
        .min_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Wispou DebugUI");
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                ui.label(format!("RAW FPS: {:?}", fps.value().unwrap_or(0.0).floor()));
                ui.label(format!("AVG FPS: {:?}", fps.average().unwrap_or(0.0).floor()));
                ui.label(format!("SMOOTH FPS: {:?}", fps.smoothed().unwrap_or(0.0).floor()));
            }
            ui.separator();
            ui.label("Player");
            ui.label(format!("Pixel X: {:?}", transform_player.translation().x));
            ui.label(format!("Pixel Y: {:?}", transform_player.translation().y));
            ui.label(format!("World X: {:?}", (transform_player.translation().x / 64.0)));
            ui.label(format!("World Y: {:?}", (transform_player.translation().y / 64.0)));
        });
}

pub fn start_menu(mut contexts: EguiContexts,) {
    //egui::Window::new("Wispou")
}