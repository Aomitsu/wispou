/*
    ”Tout petit déjà, on se demande en sois, comment tourne le monde si il en a le choix.
        A voir ces adultes sur leur chemin de croix, si c'est bien l'Homme le maître, si il n'est pas la proie.
        Jeune ado voilà que l'on se prends pour le roi, on se dit que notre destin, nous en traçeront la voie.
        Sur notre trône de zinc nous nous tenons bien droit, assez fort pour dompter le grand cheval de bois,
        Oui mais voici le jour venu et même en avance, le moment tant attendu de rentrer dans la dance,
        Nous avions convenus d'une vie de jouissance et nous voilà dans les rues, perds dans l'espérance.
        Ce qui est advenu, nul n'en avait conscience, notre vie nous échappe et même la Science,
        est inéfficace à ravire notre conscience, elle a perdue sa place, en choquant son audience.

        La vie est belle,
        Dans ses joies comme ses chaînes,
        Elle nous appelle, et nous lâche dans l'arène,
        La vie est telle, le sang qui pulse dans nos veines
            Elle nous rappèle que c'est ce qu'elle en vaut la peine[...]”
                                                                                - Titouan, La vie est Belle
*/

use std::collections::HashMap;

use bevy::{prelude::*};
use rand::prelude::*;

use super::{
    chunk::Chunk,
    block::{BlockType, Block}
};



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

#[derive(Component)]
pub struct WorldComponent;
#[derive(Component, Debug, Clone)]
pub struct World {
    seed: i32,
    map_type: MapType,
    chunks: HashMap<i32, Chunk>,
    entity: Entity,
}


/// Gestion du Monde
/// 
/// Officiellement le pire code de ma carrière
impl World {
    pub fn new(map_type: MapType, mut seed: Option<i32>, commands: &mut Commands) -> Self {
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

    pub fn load_chunk(
        &mut self,
        chunk_id: i32,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) -> &mut Self {
        if self.chunk_exists(chunk_id) {
            if let Some(chunk) = self.chunks.get_mut(&chunk_id) {
                if chunk.entity.is_none() {
                    chunk.entity = Some(chunk.generate(commands, self.entity, &asset_server));
                    debug!("Loaded chunk {}", chunk_id);
                }
            } else {
                panic!("Chunk doesn't exist")
            }
        } else {
            self.generate_chunk(chunk_id)
                .load_chunk(chunk_id, commands, asset_server);
        }
        self
    }
    
    pub fn unload_chunk(&mut self, chunk_id: i32, commands: &mut Commands) -> &mut Self {
        // TODO: Put this code into Chunk
        if self.chunk_exists(chunk_id) {
            if let Some(chunk) = self.chunks.get_mut(&chunk_id) {
                if let Some(chunk_entity) = chunk.entity {
                    commands
                        .get_entity(chunk_entity)
                        .unwrap()
                        .despawn_recursive();
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
                chunk
                    .blocks
                    .get(&IVec2::new(&coord.x / CHUNK_SIZE, &coord.y / CHUNK_SIZE))
                    .unwrap()
                    .clone()
            } else {
                Block::air(coord)
            }
        } else {
            Block::air(coord)
        }
    }
}

