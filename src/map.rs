use std::collections::HashMap;

use bevy::{prelude::*, ecs::query::WorldQuery};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::global::GlobalRessources;

pub const CHUNK_SIZE: i32 = 16;
pub const BLOCK_SIZE: i32 = 64;

#[derive(Debug, Clone)]
pub struct PlayerCoords {
    pub x: f32,
    pub y: f32,
    pub chunk_id: i32,
}

#[derive(Debug, Clone)]
pub enum MapType {
    Flat,
    Perlin,
}
#[derive(Clone, Copy, Debug)]
pub enum BlockType {
    Dirt,
    Grass,
    Air,
}

#[derive(Component)]
pub struct WorldComponent;
#[derive(Component, Debug, Clone)]
pub struct World {
    seed: i32,
    map_type: MapType,
    chunks: HashMap<i32, Chunk>,
    entity: Entity,
}
#[derive(Component)]
pub struct ChunkComponent;
#[derive(Debug, Clone)]
pub struct Chunk {
    world_relative_vec: IVec2,
    blocks: HashMap<IVec2, Block>,
    chunk_id: i32,
    /// Doesn't exist if the chunk is not loaded
    entity: Option<Entity>
}
#[derive(Component)]
pub struct BlockComponent;

#[derive(Debug, Clone)]
pub struct Block {
    chunk_relative_vec: IVec2,
    block_type: BlockType,
}

impl World {

    pub fn new(map_type: MapType, mut seed: Option<i32>, mut commands: &mut Commands) -> Self {
        let mut rng = rand::thread_rng();
        if seed.is_none() {
            seed = Some(rng.gen_range((10 * 10_i32.pow(2))..(10 * 10_i32.pow(8))));
        };

        let entity_id = commands.spawn(WorldComponent).id();

        Self {
            seed: seed.unwrap(),
            map_type,
            chunks: HashMap::new(),
            entity: entity_id,
        }
        
    }
    
    pub fn generate_chunk(&mut self, chunk_id: i32) -> &mut Self {
        if self.chunks.contains_key(&chunk_id) {
            panic!("Chunk already exists")
        }
        match self.map_type {
            MapType::Flat => {
                let mut chunk = Chunk::new(chunk_id);
                chunk.fill_blocks(
                    BlockType::Grass,
                    Vec2::new(0.0, 64.0),
                    Vec2::new(15.0, 64.0),
                );
                chunk.fill_blocks(BlockType::Dirt, Vec2::new(0.0, 60.0), Vec2::new(15.0, 63.0));

                self.chunks.insert(chunk_id, chunk);
                debug!("Generated chunk {}", chunk_id);
            }
            MapType::Perlin => todo!("Perlin generation is not possible yet"),
        }
        self
    }
    pub fn chunk_exists(&self, chunk_id: i32) -> bool {
        self.chunks.contains_key(&chunk_id)
    }

    /*pub fn update(&self, mut commands: &mut Commands, asset_server: Res<AssetServer>) {
        for ele in &self.chunks {
            ele.1.generate(&mut commands, self.entity, &asset_server);
        }
    }*/
    pub fn load_chunk(&mut self, chunk_id: i32, commands: &mut Commands, asset_server: &Res<AssetServer>) -> &mut Self {
        if self.chunk_exists(chunk_id) {
            if let Some(chunk) = self.chunks.get_mut(&chunk_id) {
                if chunk.entity.is_none() {
                    chunk.entity = Some( chunk.generate(commands, self.entity, &asset_server));
                    debug!("Loaded chunk {}", chunk_id);
                }
            } else {
                panic!("Chunk doesn't exist")
            }
        } else {
            self.generate_chunk(chunk_id).load_chunk(chunk_id, commands, asset_server);
        }
        self
    }
    pub fn unload_chunk(&mut self, chunk_id: i32,  commands: &mut Commands) -> &mut Self  {
        if self.chunk_exists(chunk_id) {
            if let Some(chunk) = self.chunks.get_mut(&chunk_id) {
                if let Some(chunk_entity) = chunk.entity {
                    commands.get_entity(chunk_entity).unwrap().despawn_recursive();
                    chunk.entity = None;
                    debug!("Unloaded chunk {}", chunk_id);
                }
            }
        }
        self
    }
    pub fn get_block(&self, coord: IVec2) -> Block {
        let block_chunk = (coord.x as f32 / CHUNK_SIZE as f32).floor() as i32;
        if self.chunk_exists(block_chunk) {
            if let Some(chunk) = self.chunks.get(&block_chunk) {
                chunk.blocks.get(&IVec2::new(&coord.x / CHUNK_SIZE, &coord.y / CHUNK_SIZE)).unwrap().clone()
            } else {
                Block::air(coord)
            }
        } else {
            Block::air(coord)
        }
    }
}

impl Chunk {
    pub fn new(chunk_id: i32) -> Self {
        Self {
            world_relative_vec: IVec2 {
                x: chunk_id as i32 * 16,
                y: 0,
            },
            blocks: HashMap::new(),
            chunk_id,
            entity: None,
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
    pub fn generate(
        &mut self,
        mut commands: &mut Commands,
        world_entity: Entity,
        asset_server: &Res<AssetServer>,
    ) -> Entity {
        let texture_dirt_handle: Handle<Image> = asset_server.load("dirt.png");
        let texture_grass_handle: Handle<Image> = asset_server.load("grass.png");
        //commands.get_entity(world_entity).unwrap().add_child(child);
        let chunk_entity = commands
            .spawn((
                SpatialBundle {
                    //transform: Transform::from_xyz(self.chunk_id as f32 * 16.0, -200.0, 0.0),
                    transform: Transform::from_xyz(
                        (64.0 * 16.0) * self.chunk_id as f32,
                        -200.0,
                        0.0,
                    ),
                    ..default()
                },
                ChunkComponent,
            ))
            .id();
        for (coord, block) in &self.blocks {
            let temp_block = commands
                .spawn((
                    SpriteBundle {
                        texture: match block.block_type {
                            BlockType::Dirt => texture_dirt_handle.clone(),
                            BlockType::Grass => texture_grass_handle.clone(),
                            _ => texture_dirt_handle.clone(),
                        },
                        //transform: Transform::from_xyz(coord.x as f32 * 32.0, coord.y as f32 * 32.0, 0.0),
                        transform: Transform::from_xyz(
                            coord.x as f32 * 64.0,
                            coord.y as f32 * 64.0,
                            0.0,
                        ),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid(32.0, 32.0),
                ))
                .id();
            commands
                .get_entity(chunk_entity)
                .unwrap()
                .add_child(temp_block);
        };
        self.entity = Some(chunk_entity);
        chunk_entity
    }
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
