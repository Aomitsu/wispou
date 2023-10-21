# World
Bien évidemment, ce projet est remplis d'erreurs de jeunesse ! Je vais donc essayer ici de définir la suite du projet, en ce qui concerne le monde.

L'objectif est d'avoir un monde totalement contrôlable, avec un moyen d'ajouter des assets facilement !

```Rust
pub const CHUNK_SIZE: i32 = 16;
pub const BLOCK_SIZE: i32 = 64;
```

```Rust
enum BlockType {
    Type::Dirt,     <- Block normal
    Type::Grass,    <- Se répands
    Type::Air,      <- Block par défaut
    ... ... ...
}

BlockType {             <- Pour chaque enum, les paramètres possible 
    transparent: bool,
    mining_level: i8,
    texture: Asset,

}

Block {
    block_type: BlockType,
    entity: Entity,
    ---

    fn destroy() <- Détruire le block
}
```
---
```Rust
Chunk {
    chunk_id: i32,
    blocks: HashMap<IVec2, Block> <- IVec2 = Coordonées relative au chunk
    entity: Entity,

    fn new(chunk_id) <- Créer un chunk par ID ( empty)
    fn generate()    <- Générer le chunk
    -- Suite: Coords relatif au chunk ( soit de 0 à 15)
    fn set_block(coords, Block) <- Définir un block à une coordonnée précise
    fn fill_block(coordsA, coordsB, Block)
}
```
---
```Rust
enum MapType {

}
World {
    chunks: HashMap<IVec2, Chunk> <- IVec2 = Coordonées relative au chunk
    entity: Entity,

    fn new(chunk_id, MapType, seed) <- Créer un monde
    fn load_chunk(id)               <- Charger dans la mémoire un chunk
    fn unload_chunk(id)             <- Décharger dans la mémoire un chunk, et le sauvegarder dans un fichier
    fn summon_chunk(id)             <- Faire apparaître dans le monde le chunk
    fn kill_chunk(id)               <- Supprimer le monde du Chunk
    fn reload_chunk(id)             <- Actualiser et Sauvegarder le chunk ( collision )
    fn save_chunk(id)               <- Sauvegarder le chunk
     
}
```