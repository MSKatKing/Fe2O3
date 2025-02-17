mod primitive;
mod variable;
mod optional;

use uuid::Uuid;
use data::nbt::NBT;
use data::queue::Queue;
use data::resource::Identifier;
use data::transform::{Angle, Position};
pub use primitive::*;
pub use variable::*;

pub trait PacketData {
    fn serialize(self) -> Vec<u8>;
    fn deserialize(queue: &mut Queue) -> Option<Self> where Self: Sized;
}

// TODO: text components

impl PacketData for Identifier {
    fn serialize(self) -> Vec<u8> {
        self.to_string().serialize()
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Identifier::from(String::deserialize(queue)?))
    }
}

impl PacketData for Position {
    fn serialize(self) -> Vec<u8> {
        (((self.x as i64 & 0x3FFFFFF) << 38) | ((self.z as i64 & 0x3FFFFFF) << 12) | ((self.y as i64 & 0xFFF))).serialize()
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        let val = i64::deserialize(queue)?;
        let x = (val >> 38) as i32;
        let y = ((val << 52) >> 52) as i16;
        let z = ((val << 26) >> 38) as i32;

        Some(Position::new(x, y, z))
    }
}

impl PacketData for NBT {
    fn serialize(mut self) -> Vec<u8> {
        self.as_network();
        self.as_bytes()
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        NBT::from_bytes_network(queue)
    }
}

impl PacketData for Angle {
    fn serialize(self) -> Vec<u8> {
        self.0.serialize()
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self(u8::deserialize(queue)?))
    }
}

impl PacketData for Uuid {
    fn serialize(self) -> Vec<u8> {
        self.as_u128().to_be_bytes().to_vec()
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Uuid::from_u128(queue.pop::<u128>()?))
    }
}