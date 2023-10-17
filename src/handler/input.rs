use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::player::Player;

pub fn move_character(
    keyboard: Res<Input<KeyCode>>,
    mut query_atlas_sprite: Query<&mut TextureAtlasSprite, With<Player>>,
    mut query_impulse: Query<&mut ExternalImpulse, With<Player>>,
    mut query_playervelocity: Query<&mut Velocity, With<Player>>,
) {
    let mut texture_atlas_sprite = query_atlas_sprite.single_mut();
    let mut impulse = query_impulse.single_mut();
    let mut playervelocity = query_playervelocity.single_mut();

    if keyboard.pressed(KeyCode::Left) {
        playervelocity.linvel = Vec2::new(-200.0, playervelocity.linvel.y);
        texture_atlas_sprite.flip_x = true;
    }
    if keyboard.pressed(KeyCode::Right) {
        playervelocity.linvel = Vec2::new(200.0, playervelocity.linvel.y);
        texture_atlas_sprite.flip_x = false;
    }
    if keyboard.just_pressed(KeyCode::Space) {
        impulse.impulse = Vec2::new(0.0, 400.0);
    }

    /*if keyboard.just_released(KeyCode::Left) {
        texture_atlas_sprite.flip_x = false;
    }*/
}
