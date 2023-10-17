use crate::map::{PlayerCoords, World};
use bevy::prelude::*;

#[derive(Resource, Default, Clone)]
pub struct GlobalRessources {
    pub world: Option<World>,
    pub player_coords: Option<PlayerCoords>,
}
impl GlobalRessources {
    pub fn new() -> GlobalRessources {
        GlobalRessources {
            world: None,
            player_coords: None,
        }
    }
}
