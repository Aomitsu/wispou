use bevy::prelude::*;

pub mod world;
pub mod chunk;
pub mod block;

pub const CHUNK_SIZE: i32 = 16;
pub const BLOCK_SIZE: i32 = 64;

#[derive(Debug, Clone)]
pub enum MapType {
    Flat,
    Perlin,
}

#[derive(Component)]
pub struct WispouWorldComponent;

#[derive(Component)]
pub struct ChunkComponent;

#[derive(Component)]
pub struct BlockComponent;