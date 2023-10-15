use bevy_egui::{EguiContexts, egui};


pub fn debug_ui(mut contexts: EguiContexts) {
    egui::SidePanel::left("Hello")
    .resizable(false)
    .show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}