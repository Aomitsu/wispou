use std::collections::HashMap;

use bevy::prelude::*;
use rand::prelude::*;

pub enum MapType {
    Flat,
    Perlin,
}
#[derive(Clone, Copy)]
pub enum BlockType {
    Dirt,
    Grass,
}

#[derive(Component)]
pub struct WorldComponent;
pub struct World {
    seed: i32,
    map_type: MapType,
    chunks: HashMap<i32, Chunk>,
    entity: Entity,
}
#[derive(Component)]
pub struct ChunkComponent;
///
pub struct Chunk {
    world_relative_vec: IVec2,
    blocks: HashMap<IVec2, Block>,
}
#[derive(Component)]
pub struct BlockComponent;
pub struct Block {
    chunk_relative_vec: IVec2,
    block_type: BlockType,
}

impl World {
    pub fn new(map_type: MapType, mut seed: Option<i32>, mut commands: Commands) -> Self {
        let mut rng = rand::thread_rng();
        if seed.is_none() {
            seed = Some(rng.gen_range(10 * 10 ^ 5..10 * 10 ^ 25));
        };

        let entity_id = commands.spawn(WorldComponent).id();

        Self {
            seed: seed.unwrap(),
            map_type,
            chunks: HashMap::new(),
            entity: entity_id,
        }
    }
    pub fn generate_chunk(&mut self, chunk_id: i32) {
        if self.chunks.contains_key(&chunk_id) {
            panic!("Chunk already exists")
        }
        match self.map_type {
            MapType::Flat => {
                let mut chunk = Chunk::new(chunk_id);
                chunk
                    .fill_blocks(
                        BlockType::Grass,
                        Vec2::new(0.0, 64.0),
                        Vec2::new(15.0, 64.0),
                    )
                    .fill_blocks(BlockType::Dirt, Vec2::new(0.0, 63.0), Vec2::new(15.0, 60.0));
            }
            MapType::Perlin => todo!("Perlin generation is not possible yet"),
        }
    }
}

impl Chunk {
    pub fn new(chunk_id: i32) -> Self {
        Self {
            world_relative_vec: IVec2 {
                x: chunk_id as i32 * 64,
                y: 0,
            },
            blocks: HashMap::new(),
        }
    }
    pub fn fill_blocks(&mut self, block_type: BlockType, from: Vec2, to: Vec2) -> &mut Self {
        for x in from.x.floor() as i32..=to.x.floor() as i32 {
            for y in from.y.floor() as i32..=to.y.floor() as i32 {
                let coord = IVec2::new(x as i32, y as i32);
                let block = Block::new(block_type.clone(), coord);
                self.blocks.insert(coord, block);
            }
        }
        self
    }
}

impl Block {
    pub fn new(block_type: BlockType, chunk_coord: IVec2) -> Self {
        Self {
            chunk_relative_vec: chunk_coord,
            block_type,
        }
    }
}
