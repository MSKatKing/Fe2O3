mod section;
mod heightmap;

use std::collections::HashMap;
use fe2o3_nbt::{compound_nbt, NBT};
use packet::Buffer;
use crate::game::Location;
use crate::game::world::chunk::section::ChunkSection;
use heightmap::HeightMap;
use crate::networking::packet::play::ChunkDataAndUpdateLight;

#[derive(Hash, PartialEq, Eq)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32
}

pub struct ChunkManager {
    chunks: HashMap<ChunkPosition, Chunk>
}

pub struct Chunk {
    chunk_sections: [ChunkSection; 24],

    hm_motion_blocking: HeightMap,
    hm_world_surface: HeightMap
}

impl Chunk {
    pub fn empty() -> Self {
        Self {
            chunk_sections: Default::default(),
            hm_motion_blocking: HeightMap::new(),
            hm_world_surface: HeightMap::new()
        }
    }

    pub fn flat_generation() -> Self {
        let mut out = Self::empty();

        for x in 0..16 {
            for z in 0..16 {
                for y in 1..4 {
                    out.set_block(Location::new(x as _, y as _, z as _), 10)
                }

                out.set_block(Location::new(x as _, 4.0, z as _), 9);
                out.set_block(Location::new(x as _, 0.0, z as _), 79);
            }
        }

        out
    }

    pub fn set_block(&mut self, pos: Location, block: u8) {
        let y = (pos.y() + 64.0) as u64;

        if y > self.hm_world_surface.get_height(pos.x() as i32, pos.z() as i32) as u64 {
            self.hm_world_surface.set_height(pos.x() as _, pos.z() as _, y as _);
        }

        let section = y / 16;
        self.chunk_sections[section as usize].set_block(pos, block);
    }

    pub fn get_heightmap_nbt(&self, network: bool) -> NBT {
        let mut nbt = NBT::new(network);

        nbt.write_tag("", compound_nbt!(
            "WORLD_SURFACE" => self.hm_world_surface.generate_data(),
            "MOTION_BLOCKING" => self.hm_motion_blocking.generate_data()
        ));

        nbt
    }
}

impl Into<ChunkDataAndUpdateLight> for (&ChunkPosition, &Chunk) {
    fn into(self) -> ChunkDataAndUpdateLight {
        let mut data = Buffer::new();

        for section in &self.1.chunk_sections {
            data.write(<&ChunkSection as Into<Vec<u8>>>::into(section).as_slice());
        }

        ChunkDataAndUpdateLight {
            x: self.0.x,
            z: self.0.z,
            heightmaps: self.1.get_heightmap_nbt(true),
            data: data.buffer,
            block_entities: vec![],
            sky_light_mask: vec![],
            block_light_mask: vec![],
            empty_sky_light_mask: vec![],
            empty_block_light_mask: vec![],
            sky_light_array: vec![],
            block_light_array: vec![],
        }
    }
}