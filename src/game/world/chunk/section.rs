use std::collections::HashMap;
use fe2o3_nbt::pack_entries;
use packet::{Buffer, VarInt};
use crate::game::Location;

#[derive(Default)]
pub struct ChunkSection {
    blocks: HashMap<Location, u8>,
}

impl ChunkSection {
    pub fn empty() -> ChunkSection {
        Self {
            blocks: HashMap::new()
        }
    }

    pub fn set_block(&mut self, location: Location, block: u8) {
        self.blocks.insert(location, block);
    }

    pub fn get_block(&self, location: Location) -> &u8 {
        &self.blocks[&location]
    }
}

impl Into<Vec<u8>> for &ChunkSection {
    fn into(self) -> Vec<u8> {
        let mut out = Buffer::new();

        let block_count: u16 = self.blocks.len() as u16;

        let mut all_same = None;
        if block_count >= (16 * 16 * 16) {
            all_same = Some(*self.get_block(Location::new(0.0, 0.0, 0.0)));
            self.blocks.iter().for_each(|(_, b)| if *b as i32 != all_same.unwrap_or(0) as i32 { all_same = None });
        }

        out.write(block_count);

        if block_count == 0 || all_same.is_some() {
            let block = all_same.unwrap_or(0);
            out.write(0u8);
            out.write(VarInt(block as i32));
            out.write(VarInt(0));
        } else {
            out.write(15u8);
            let mut blocks = Vec::new();
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        blocks.push(*self.blocks.get(&Location::new(x as f64, y as f64, z as f64)).unwrap_or(&0) as i32);
                    }
                }
            }

            let flattened = pack_entries(&blocks[..], 15);
            out.write(VarInt(flattened.len() as i32));
            flattened.iter().for_each(|v| out.write(*v));
        }

        out.write(0u8);
        out.write(VarInt(0));
        out.write(VarInt(0));

        out.buffer
    }
}