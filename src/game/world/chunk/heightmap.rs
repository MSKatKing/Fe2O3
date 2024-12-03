use fe2o3_nbt::{pack_entries, NBTTag};

const HM_BIT_PER_ENTRY: usize = 9;

pub struct HeightMap {
    data: [i32; 256]
}

impl HeightMap {
    pub fn new() -> Self {
        Self {
            data: [0i32; 256]
        }
    }

    pub fn generate_data(&self) -> NBTTag {
        NBTTag::LongArray(pack_entries(&self.data, HM_BIT_PER_ENTRY))
    }

    pub fn set_height(&mut self, x: i32, z: i32, height: i32) {
        self.data[(z * 16 + x) as usize] = height;
    }

    pub fn get_height(&self, x: i32, z: i32) -> i32 {
        self.data[(z * 16 + x) as usize]
    }
}