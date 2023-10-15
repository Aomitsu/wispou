use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use dotenv::dotenv;

mod map;

#[derive(Component)]
struct Player;

fn main() {
    dotenv().ok();
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, move_character)
        .run();
}

fn setup(mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ) {

    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("playermodel.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 128.0),  3,1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        ..default()
    }, Player,
    RigidBody::Dynamic,
    LockedAxes::ROTATION_LOCKED,
    ExternalImpulse::default(),
    GravityScale(9.81),
    Restitution::coefficient(0.7),
    Collider::cuboid(32.0, 64.0),
    KinematicCharacterController {
        snap_to_ground: Some(CharacterLength::Absolute(0.5)),
        ..default()
    }));

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
) {
    let mut transform = query.single_mut();
    let mut texture_atlas_sprite = query2.single_mut();
    let mut impulse = query3.single_mut();
    if keyboard.pressed(KeyCode::Left) {
        transform.translation = Some(Vec2::new(-1.0, 0.0));
        texture_atlas_sprite.flip_x = true;
    }
    if keyboard.pressed(KeyCode::Right) {
        transform.translation = Some(Vec2::new(1.0, 0.0));
        texture_atlas_sprite.flip_x = false;
    }
    if keyboard.just_pressed(KeyCode::Space) {
        impulse.impulse = Vec2::new(0.0, 500.0);
    }
}