/*
    “Ce n'est pas le projet qui m'intéresse, c'est la vie.”
                                        - Gérard Depardieu mdr lol
*/

use bevy::prelude::*;

use crate::map::{world::{WispouWorld}, PlayerCoords};

/// GlobalRessources
/// 
/// Variable globale accessible par tout les systèmes, en local.
/// 
/// Ici on y stock le monde, ainsi que les coordonnées du joueur local pour les rendre accessible facilement.
#[derive(Resource, Default, Clone)]
pub struct GlobalRessources {
    pub world: Option<WispouWorld>,
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
