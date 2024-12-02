use std::collections::HashMap;
use fe2o3_nbt::NBT;
use crate::game::world::heightmap::HeightMap;

#[derive(Hash, PartialEq, Eq)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32
}

pub struct ChunkManager {
    chunks: HashMap<ChunkPosition, Chunk>
}

pub struct Chunk {

}

pub fn empty_heightmap() -> NBT {
    let mut nbt = NBT::new(true);

    nbt.write_tag("MOTION_BLOCKING", HeightMap::new().generate_data());
    nbt.write_tag("WORLD_SURFACE", HeightMap::new().generate_data());

    nbt
}