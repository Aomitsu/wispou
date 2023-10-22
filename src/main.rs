/*
    ”Mon projet préféré ? C'est le prochain.”
                             - Frank Lloyd Wright
*/

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use dotenv::dotenv;
use global::GlobalRessources;
use map::world::{WispouWorld};

use crate::handler::player;

mod global;
mod handler;
mod map;
mod ui;
mod utils;

#[derive(Component)]
struct Camera;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
enum AppState {
    MainMenu,
    InGame
}
impl Default for AppState {
    fn default() -> Self {
        AppState::MainMenu
    }
}

/// Wispou
/// 
/// An epic 2D game, an improved, rust-based recreation of a video game created by [Aywen](https://www.youtube.com/@aywenvideos) 
fn main() {
    dotenv().ok();
    print!("Hello Wispou ! V 0.0.1");
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Immediate,
                        resizable: false,
                        title: "Wispou".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // App state, pour gérer le menu et le jeu différemment
        .add_state::<AppState>()
        // Global ressources //
        .insert_resource(global::GlobalRessources::new())
        // Plugins //
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // STARUP //
        .add_systems(Startup, setup)
        // GAME Systems //
        
        .add_systems(Startup, handler::player::spawn_player)
        .add_systems(
            Update,
            (
                handler::input::move_character,
                handler::camera::update_camera,
                ui::debug_ui,
                player::update_player,
                player::update_player_world,
            ),
        )

        // MAIN MENU SYSTEM //

        //.add_systems(Update, ui::start_menu)

        // Let's f-cking go !!
        .run();
}
fn setup(mut commands: Commands, mut globalres: ResMut<GlobalRessources>) {
    commands.spawn((Camera2dBundle::default(), Camera));
    // Create the map::World instance
    globalres.world = Some(
        WispouWorld::new(map::MapType::Flat, None, &mut commands)
    );
}