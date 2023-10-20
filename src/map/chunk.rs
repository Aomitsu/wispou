use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use super::{block::{Block, BlockType}, world::World};

#[derive(Component)]
pub struct ChunkComponent;
#[derive(Debug, Clone)]
pub struct Chunk {
    pub world_relative_vec: IVec2,
    pub blocks: HashMap<IVec2, Block>,
    pub chunk_id: i32,
    /// Doesn't exist if the chunk is not loaded
    pub block_entity: Option<HashMap<IVec2, Entity>>,
    pub entity: Option<Entity>,
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
            block_entity: None,
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
        commands: &mut Commands,
        _world_entity: Entity,
        asset_server: &Res<AssetServer>,
    ) -> Entity {
        self.block_entity = Some(HashMap::new());
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
            self.block_entity.as_mut().unwrap().insert(*coord, temp_block);
        }
        self.entity = Some(chunk_entity);
        chunk_entity
    }
    pub fn reload_collisions(&mut self, commands: &mut Commands, world: World) {
    }
}

