use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use super::{
    block::{Block, BlockType},
    MapType, CHUNK_SIZE,
};

#[derive(Debug, Clone)]
pub struct Chunk {
    pub chunk_id: i32,
    pub blocks: HashMap<IVec2, Block>,
    pub entity: Option<Entity>,
}

impl Chunk {
    pub fn new(chunk_id: i32) -> Self {
        Self {
            chunk_id,
            blocks: HashMap::new(),
            entity: None,
        }
    }
    pub fn generate(&mut self, map_type: MapType, seed: i32) -> &mut Self {
        match map_type {
            MapType::Flat => {
                self.fill_blocks(
                    BlockType::grass(),
                    IVec2::new(0, 15),
                    IVec2::new(CHUNK_SIZE - 1, 15),
                );
                self.fill_blocks(
                    BlockType::dirt(),
                    IVec2::new(0, 12),
                    IVec2::new(CHUNK_SIZE - 1, 14),
                );
            }
            MapType::Perlin => todo!("Perlin"),
        }
        self
    }
    pub fn fill_blocks(&mut self, block_type: BlockType, from: IVec2, to: IVec2) -> &mut Self {
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                let coord = IVec2::new(x as i32, y as i32);
                let block = Block::new(block_type.clone(), coord);
                self.blocks.insert(coord, block);
            }
        }
        self
    }
    pub fn set_blocks() {}
    /*pub fn fill_blocks(&mut self, block_type: BlockType, from: Vec2, to: Vec2) -> &mut Self {}
    pub fn generate(&mut self,commands: &mut Commands,_world_entity: Entity,asset_server: &Res<AssetServer>,) -> Entity {}
    pub fn reload_collisions(&mut self, commands: &mut Commands, world: World) {
        */
}
