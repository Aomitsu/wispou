use std::collections::HashMap;

use bevy::prelude::*;
use rand::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone)]
pub enum MapType {
    Flat,
    Perlin,
}
#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum BlockType {
    Dirt,
    Grass,
}

#[derive(Component)]
pub struct WorldComponent;
#[derive(Component)]
#[derive(Debug, Clone)]
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
    chunk_id: i32
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
            seed = Some(rng.gen_range((10 * 10 ^ 5)..(10 * 10 ^ 25)));
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
                    chunk.fill_blocks(
                        BlockType::Dirt, 
                        Vec2::new(0.0, 60.0), 
                        Vec2::new(15.0, 63.0)
                    );
                
                self.chunks.insert(chunk_id, chunk);
            }
            MapType::Perlin => todo!("Perlin generation is not possible yet"),
        }
        self
    }

    pub fn update(&self, mut commands: &mut Commands, asset_server: Res<AssetServer>) {
        for ele in &self.chunks {
            ele.1.generate(
                &mut commands,
                self.entity,
                &asset_server
            )
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
            chunk_id
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
    pub fn generate(&self, mut commands: &mut Commands, world_entity: Entity, asset_server: &Res<AssetServer>){
        let texture_grass_handle: Handle<Image> = asset_server.load("dirt.png");
        //commands.get_entity(world_entity).unwrap().add_child(child);
        let chunk_entity = commands.spawn((SpatialBundle{
            //transform: Transform::from_xyz(self.chunk_id as f32 * 16.0, -200.0, 0.0),
            transform: Transform::from_xyz((64.0*16.0) * self.chunk_id as f32, -200.0, 0.0),
            ..default()
        }, ChunkComponent)).id();
        for (coord, block) in &self.blocks {
            println!("{:?}", coord);
            let temp_block = commands.spawn((
                SpriteBundle {
                    texture: texture_grass_handle.clone(),
                    //transform: Transform::from_xyz(coord.x as f32 * 32.0, coord.y as f32 * 32.0, 0.0),
                    transform: Transform::from_xyz(coord.x as f32 * 64.0, coord.y as f32 * 64.0, 0.0),
                    ..default()
                },
                RigidBody::Fixed,
                Collider::cuboid(32.0, 32.0)
            )).id();
            commands.get_entity(chunk_entity).unwrap().add_child(temp_block);
        }
    }
    pub fn id(&self) -> &Self {
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
