use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::Player;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("playermodel.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 128.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(0.0, 4000.0, 0.0),
            ..default()
        },
        Player,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        ExternalImpulse::default(),
        Velocity::default(),
        GravityScale(9.81),
        Collider::capsule_y(32.0, 31.0),
        KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(1.5)),
            ..default()
        },
    ));
}
