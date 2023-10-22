use bevy::prelude::*;

pub mod world;
pub mod chunk;
pub mod block;

pub const CHUNK_SIZE: i32 = 16;
pub const BLOCK_SIZE: i32 = 64;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct PlayerCoords {
    pub x: f32,
    pub y: f32,
    pub chunk_id: i32,
}