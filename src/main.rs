use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use dotenv::dotenv;

mod map;
mod ui;
mod handler;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
struct Camera;

fn main() {
    dotenv().ok();
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, handler::player::spawn_player))
        .add_systems(Update, (move_character, handler::camera::update_camera, ui::debug_ui))
        .run();
}

fn setup(mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {

    commands.spawn((Camera2dBundle::default(), Camera));

    let texture_grass_handle: Handle<Image> = asset_server.load("dirtydancing.png");
    
    // loop 10 times
    for i in 0..10 {
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



}

fn move_character(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut KinematicCharacterController>,
    mut query2: Query<&mut TextureAtlasSprite, With<Player>>,
    mut query3: Query<&mut ExternalImpulse, With<Player>>,
    mut query_playervelocity: Query<&mut Velocity, With<Player>>,
) {
    let mut texture_atlas_sprite = query2.single_mut();
    let mut impulse = query3.single_mut();
    let mut playervelocity = query_playervelocity.single_mut();
    if keyboard.pressed(KeyCode::Left) {
        //transform.translation = Some(Vec2::new(-1.0, 0.0));
        playervelocity.linvel = Vec2::new(-100.0, playervelocity.linvel.y);
        texture_atlas_sprite.flip_x = true;
    }
    if keyboard.pressed(KeyCode::Right) {
        //transform.translation = Some(Vec2::new(1.0, 0.0));
        playervelocity.linvel = Vec2::new(100.0, playervelocity.linvel.y);
        texture_atlas_sprite.flip_x = false;
    }
    if keyboard.just_pressed(KeyCode::Space) {
        impulse.impulse = Vec2::new(0.0, 500.0);
    }
}

