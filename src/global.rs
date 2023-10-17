use bevy::prelude::*;
use crate::map::World;

#[derive(Resource, Default)]
pub struct GlobalRessources {
    pub world: Option<World>
}
impl GlobalRessources {
    pub fn new() -> GlobalRessources {
        GlobalRessources { 
            world: None
         }
    }
}