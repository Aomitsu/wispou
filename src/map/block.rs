use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum BlockType {
    Dirt,
    Grass,
    Air,
}

#[derive(Component)]
pub struct BlockComponent;

#[derive(Debug, Clone)]
pub struct Block {
    pub chunk_relative_vec: IVec2,
    pub block_type: BlockType,
}

impl Block {
    pub fn new(block_type: BlockType, chunk_coord: IVec2) -> Self {
        Self {
            chunk_relative_vec: chunk_coord,
            block_type,
        }
    }
    pub fn air(chunk_coord: IVec2) -> Self {
        Self {
            chunk_relative_vec: chunk_coord,
            block_type: BlockType::Air,
        }
    }
}
