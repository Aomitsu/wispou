use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use dotenv::dotenv;
use map::MapType;

mod handler;
mod map;
mod ui;
mod utils;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
struct Camera;

fn main() {
    dotenv().ok();

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
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), Camera));

    let mut binding = map::World::new(MapType::Flat, None, &mut commands);
    let mut test = binding.generate_chunk(-1);
    test = binding.generate_chunk(0);

    test.update(&mut commands, asset_server);

    println!("{:?}", test);

    /*  loop 10 times
    for i in 0..35 {
        // transform i to f32
        let i = i as f32;
        commands.spawn((
            SpriteBundle {
                texture: texture_grass_handle.clone(),
                transform: Transform::from_xyz(-64.0 * i, -200.0, 0.0),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(32.0, 32.0)
        ));
    }
    for i in 0..5 {
        // transform i to f32
        let i = i as f32;
        commands.spawn((
            SpriteBundle {
                texture: texture_grass_handle.clone(),
                transform: Transform::from_xyz(64.0 * i, 000.0, 0.0),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(32.0, 32.0)
        ));
    }*/
}
