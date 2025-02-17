use crate::data::PacketData;
use data::queue::Queue;

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

#[repr(transparent)]
#[derive(Default, Clone, Copy)]
pub struct VarInt(pub i32);

#[repr(transparent)]
#[derive(Default, Clone, Copy)]
pub struct VarLong(pub i64);

impl PacketData for VarInt {
    fn serialize(mut self) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            if (self.0 as u8 & !SEGMENT_BITS) == 0 {
                out.push(self.0 as u8);
                return out;
            }

            out.push((self.0 as u8 & SEGMENT_BITS) | CONTINUE_BIT);

            self.0 = self.0 >> 7;
        }
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        let mut value = 0;
        let mut position = 0;

        loop {
            let byte = queue.pop::<u8>()?;
            value = value | (((byte & SEGMENT_BITS) as i32) << position);

            if (byte & CONTINUE_BIT) == 0 { return Some(VarInt(value)); }

            position = 7;

            if position >= 32 {
                return None;
            }
        }
    }
}

impl PacketData for VarLong {
    fn serialize(mut self) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            if (self.0 as u8 & !SEGMENT_BITS) == 0 {
                out.push(self.0 as u8);
                return out;
            }

            out.push((self.0 as u8 & SEGMENT_BITS) | CONTINUE_BIT);

            self.0 >>= 7
        }
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        let mut value = 0;
        let mut position = 0;

        loop {
            let byte = queue.pop::<u8>()?;
            value = value | ((byte & SEGMENT_BITS) << position) as i64;

            if (byte & CONTINUE_BIT) == 0 {
                return Some(VarLong(value));
            }

            position += 7;

            if position >= 64 {
                return None;
            }
        }
    }
}