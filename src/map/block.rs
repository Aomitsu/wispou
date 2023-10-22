use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct BlockType {
    pub transparent: Option<bool>,
    pub mining_level: Option<i8>,
    pub texture: Option<String>,
}

impl Default for BlockType {
    fn default() -> Self {
        Self {
            transparent: Some(true),
            mining_level: None,
            texture: None,
        }
    }
}
impl BlockType {
    pub fn grass() -> Self {
        Self {
            transparent: Some(false),
            mining_level: Some(0),
            texture: Some(String::from("grass.png")),
        }
    }
    pub fn dirt() -> Self {
        Self {
            transparent: Some(false),
            mining_level: Some(0),
            texture: Some(String::from("dirt.png")),
        }
    }
}

#[derive(Component)]
pub struct BlockComponent;

#[derive(Debug, Clone)]
pub struct Block {
    pub block_type: BlockType,
    pub entity: Option<Entity>,
}

impl Block {
    pub fn new(block_type: BlockType, chunk_coord: IVec2) -> Self {
        Self {
            block_type,
            entity: None,
        }
    }
    pub fn air(chunk_coord: IVec2) -> Self {
        Self {
            block_type: BlockType::default(),
            entity: None,
        }
    }
    pub fn destroy() {
        todo!("destroy block")
    }
}
