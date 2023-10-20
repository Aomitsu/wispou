/*
    ”Insomnie d'amour, plus chaude a minuit qu'au soleil en pleins jour”
                                                        - Aladdin, 1992
*/

use bevy::{prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    global::GlobalRessources,
    map::world::{PlayerCoords, BLOCK_SIZE, CHUNK_SIZE},
};

#[derive(Component)]
pub struct Player;

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

pub fn update_player(
    mut query_gt_player: Query<&mut GlobalTransform, With<Player>>,
    mut globaldata: ResMut<GlobalRessources>,
) {
    let transform_player = query_gt_player.single_mut();
    let player_world_x = transform_player.translation().x / BLOCK_SIZE as f32;
    let player_world_y = transform_player.translation().y / BLOCK_SIZE as f32;
    let player_actual_chunk =
        ((transform_player.translation().x / BLOCK_SIZE as f32) / CHUNK_SIZE as f32).floor();
    globaldata.player_coords = Some(PlayerCoords {
        x: player_world_x,
        y: player_world_y,
        chunk_id: player_actual_chunk as i32,
    })
}

pub fn update_player_world(
    mut globaldata: ResMut<GlobalRessources>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let player_data = globaldata.clone().player_coords;
    let world = globaldata.world.as_mut();

    if let Some(world) = world {
        if let Some(player_data) = player_data {
            let actual_chunk = player_data.chunk_id;
            world
                .load_chunk(actual_chunk, &mut commands, &asset_server)
                .load_chunk(actual_chunk + 1, &mut commands, &asset_server)
                .load_chunk(actual_chunk - 1, &mut commands, &asset_server)
                .unload_chunk(actual_chunk + 2, &mut commands)
                .unload_chunk(actual_chunk - 2, &mut commands);
        }
    }
}
