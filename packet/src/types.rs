use std::fmt::{Debug, Display, Formatter};
use crate::{Buffer, Deserializable, Serializable, VarInt};

pub struct ByteArrayPrefixedLength(pub Vec<u8>);

impl Display for ByteArrayPrefixedLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.0.clone())
            .unwrap()
            .trim())
    }
}

impl Serializable for ByteArrayPrefixedLength {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(VarInt(self.0.len() as i32));

        for i in 0..self.0.len() {
            buffer.write(self.0[i]);
        }
    }
}

impl Deserializable for ByteArrayPrefixedLength {
    fn deserialize(buffer: &mut Buffer) -> Self {
        let length = buffer.read::<VarInt>().0 as usize;
        let mut vec = Vec::with_capacity(length);

        for _ in 0..length {
            vec.push(buffer.read::<u8>());
        }

        Self(vec)
    }
}

pub struct ByteArrayInferredLength(pub Vec<u8>);

impl Display for ByteArrayInferredLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.0.clone())
            .unwrap()
            .trim())
    }
}

impl Serializable for ByteArrayInferredLength {
    fn serialize(self, buffer: &mut Buffer) {
        for i in 0..self.0.len() {
            buffer.write(self.0[i]);
        }
    }
}

impl Deserializable for ByteArrayInferredLength {
    fn deserialize(buffer: &mut Buffer) -> Self {
        let length = buffer.len() - buffer.cursor;
        let mut vec = Vec::with_capacity(length);

        for _ in 0..length {
            vec.push(buffer.read::<u8>());
        }

        Self(vec)
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub namespace: String,
    pub asset: String
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.asset)
    }
}
impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Identifier {
    pub fn new(namespace: impl Into<String>, asset: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            asset: asset.into()
        }
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self::new("minecraft", value)
    }
}

impl Serializable for Identifier {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(format!("{}:{}", self.namespace, self.asset));
    }
}
impl Deserializable for Identifier {
    fn deserialize(buffer: &mut Buffer) -> Self {
        let strings = buffer.read::<String>();
        strings.into()
    }
}
impl From<String> for Identifier {
    fn from(value: String) -> Self {
        let strings = value.split(":").collect::<Vec<_>>();
        if strings.len() == 2 {
            Self::new(strings[0], strings[1])
        } else if strings.len() == 1 {
            Self::new("minecraft", strings[0])
        } else {
            Self::new("minecraft", "thing")
        }
    }
}

pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Deserializable for Position {
    fn deserialize(buffer: &mut Buffer) -> Self {
        let data = buffer.read::<u64>();

        let y = (data & 0xFFF) as i16; // first 12 bits
        let z = ((data >> 12) & 0x3FFFFFF) as i32; // next 26 bits
        let x = ((data >> 38) & 0x3FFFFFF) as i32; // final 26 bits

        Self { x, y, z }
    }
}
impl Serializable for Position {
    fn serialize(self, buffer: &mut Buffer) {
        let out = self.y as u64;
        let out = ((self.z as u64) << 12) | out;
        let out = ((self.x as u64) << 38) | out;

        buffer.write(out);
    }
}