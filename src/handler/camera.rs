use bevy::prelude::*;

use super::player::Player;

pub fn update_camera(
    mut query_camera: Query<&mut Transform, With<Camera>>,
    mut query_player: Query<&mut GlobalTransform, With<Player>>,
) {
    let mut transform_camera = query_camera.single_mut();
    let transform_player = query_player.single_mut();
    transform_camera.translation = Vec3::new(
        transform_player.translation().x,
        transform_player.translation().y,
        0.0,
    );
}
