use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};

use crate::handler::player::Player;

/// UI De Debug
/// 
/// Sera disponible dans la version finale sous simple perssion du F5
/// 
/// A voir si on peux y ajouter l'UI de debug des collisions
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
            /*
                FPS / GRAPHIC ENGINE RELATED
             */
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                ui.label(format!("RAW FPS: {:?}", fps.value().unwrap_or(0.0).floor()));
                ui.label(format!(
                    "AVG FPS: {:?}",
                    fps.average().unwrap_or(0.0).floor()
                ));
                ui.label(format!(
                    "SMOOTH FPS: {:?}",
                    fps.smoothed().unwrap_or(0.0).floor()
                ));
            }
            /*
                JOUEUR
             */
            ui.separator();
            ui.label("Player");
            ui.label(format!("Pixel X: {:?}", transform_player.translation().x));
            ui.label(format!("Pixel Y: {:?}", transform_player.translation().y));
            ui.label(format!(
                "World X: {:?}",
                (transform_player.translation().x / 64.0)
            ));
            ui.label(format!(
                "World Y: {:?}",
                (transform_player.translation().y / 64.0)
            ));
            ui.label(format!(
                "Chunk: {:?}",
                ((transform_player.translation().x / 64.0) / 16.0).floor()
            ));
        });
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    
}
