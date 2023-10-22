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

use std::{collections::HashMap, path::PathBuf, str::FromStr};

use bevy::{prelude::*};
use rand::prelude::*;

use crate::map::{ChunkComponent, BLOCK_SIZE, CHUNK_SIZE};

use super::{
    chunk::Chunk,
    block::{BlockType, Block}, MapType, WispouWorldComponent
};

/// La structure du monde
#[derive(Component, Debug, Clone)]
pub struct WispouWorld {
    /// Seed, définissant la rng du monde
    seed: i32,
    /// Type de map
    map_type: MapType,
    /// Base de donnée des chunks
    chunks: HashMap<i32, Chunk>,
    /// Liste des chunks générés
    generated_chunks: Vec<i32>,
    /// Entité bevy du monde
    entity: Entity,
}


/// Gestion du Monde
/// 
/// Officiellement le pire code de ma carrière
impl WispouWorld {
    pub fn new(map_type: MapType, mut seed: Option<i32>, commands: &mut Commands) -> Self {
        let mut rng = rand::thread_rng();
        if seed.is_none() {
            seed = Some(rng.gen_range((10 * 10_i32.pow(2))..(10 * 10_i32.pow(8))));
        };

        let entity_id = commands.spawn(WispouWorldComponent).id();

        Self {
            seed: seed.unwrap(),
            map_type,
            chunks: HashMap::new(),
            entity: entity_id,
            generated_chunks: Vec::new(),
        }
    }

    pub fn load_chunk(&mut self, chunk_id: i32) -> &mut Self {
        if let Some(chunk) = self.chunks.get_mut(&chunk_id) {
            debug!("Chunk {} already loaded !", chunk_id)
        } else {
            // TODO: Check if chunk exists in save file
            // Generate then load.
            let mut new_chunk = Chunk::new(chunk_id);
            new_chunk.generate(self.map_type, self.seed);
            self.chunks.insert(chunk_id, new_chunk.clone());
            debug!("Chunk {} loaded !", chunk_id)
        }
        self
    }
    pub fn unload_chunk(&mut self, chunk_id: i32) -> &mut Self {
        if let Some(chunk) = self.chunks.get_mut(&chunk_id) {
            // TODO: self.kill_chunk()
            self.chunks.remove(&chunk_id);
            debug!("Chunk {} unloaded !", chunk_id)
        } else {
            debug!("Chunk {} already unloaded !", chunk_id)
        }
        self
    }

    pub fn summon_chunk(&mut self, chunk_id: i32, commands: &mut Commands, asset_server: &Res<AssetServer>) -> &mut Self {
        if let Some(chunk) = self.chunks.get_mut(&chunk_id) {
            
            let chunk_entity = commands.spawn((
                SpatialBundle {
                    //transform: Transform::from_xyz(self.chunk_id as f32 * 16.0, -200.0, 0.0),
                    transform: Transform::from_xyz(
                        (BLOCK_SIZE * CHUNK_SIZE * chunk_id) as f32,
                        0.0,
                        0.0,
                    ),
                    ..default()
                },
                ChunkComponent,
            )).id();
            
            for block in &chunk.blocks {
                // block.1.block_type.texture.unwrap()
                let temp_block = commands
                .spawn((
                    SpriteBundle {
                        texture: asset_server.load(block.1.block_type.texture.clone().unwrap()),
                        //transform: Transform::from_xyz(coord.x as f32 * 32.0, coord.y as f32 * 32.0, 0.0),
                        transform: Transform::from_xyz(
                            (block.0.x * BLOCK_SIZE) as f32,
                            (block.0.y * BLOCK_SIZE) as f32,
                            0.0,
                        ),
                        ..default()
                    },
                ))
                .id();
            }

            commands.get_entity(self.entity).unwrap().add_child(chunk_entity);
            debug!("Chunk {} Summoned !", chunk_id)
        } else {
            debug!("Chunk {} not Summoned ! It doesn't exist, load it first !!!", chunk_id)
        }
        self
    }



    /*pub fn generate_chunk(&mut self, chunk_id: i32) -> &mut Self {
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
*/
}

