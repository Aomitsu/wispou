use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use dotenv::dotenv;
use global::GlobalRessources;
use map::MapType;

use crate::handler::player;

mod global;
mod handler;
mod map;
mod ui;
mod utils;

#[derive(Component)]
struct Camera;

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
        .insert_resource(global::GlobalRessources::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, (setup, handler::player::spawn_player))
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
        .run();
}

fn setup(mut commands: Commands, mut globalres: ResMut<GlobalRessources>) {
    commands.spawn((Camera2dBundle::default(), Camera));
    // Create the map::World instance
    globalres.world = Some(map::World::new(MapType::Flat, None, &mut commands));
}
