use std::ops::{Deref, DerefMut};
use json::JsonValue;
use uuid::Uuid;
use crate::buffer::Buffer;

pub trait Serializable: Send + Sync {
    fn serialize(self, buffer: &mut Buffer);
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct VarInt(pub i32);

impl Deref for VarInt {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VarInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serializable for VarInt {
    fn serialize(self, buffer: &mut Buffer) {
        const SEGMENT_BIT: i32 = 0x7F;
        const CONTINUE_BIT: i32 = 0x80;

        let mut value = self.0;

        loop {
            if (value & !SEGMENT_BIT) == 0 {
                buffer.write_numeric(value as u8);
                return;
            }

            buffer.write_numeric(((value & SEGMENT_BIT) | CONTINUE_BIT) as u8);
            value = ((value as u32) >> 7) as i32;
        }
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}


impl Serializable for String {
    fn serialize(self, buffer: &mut Buffer) {
        let length = VarInt(self.len() as i32);

        buffer.write(length);
        for &b in self.as_bytes() {
            buffer.write(b);
        }
    }
}

impl Serializable for u8 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self);
    }
}

impl Serializable for u16 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self);
    }
}

impl Serializable for u32 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self);
    }
}

impl Serializable for u64 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self);
    }
}

impl Serializable for u128 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self)
    }
}

impl Serializable for usize {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self)
    }
}

impl Serializable for i8 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self);
    }
}

impl Serializable for i16 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self);
    }
}

impl Serializable for i32 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self);
    }
}

impl Serializable for i64 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self);
    }
}

impl Serializable for i128 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self)
    }
}

impl Serializable for isize {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self)
    }
}

impl Serializable for bool {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self as u8)
    }
}

impl Serializable for f32 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self)
    }
}

impl Serializable for f64 {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write_numeric(self)
    }
}

impl Serializable for Uuid {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(self.as_u128());
    }
}

impl Serializable for JsonValue {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(self.to_string());
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(VarInt(self.len() as i32));
        for item in self {
            buffer.write(item);
        }
    }
}

impl<T: Serializable> Serializable for Option<T> {
    fn serialize(self, buffer: &mut Buffer) {
        match self {
            Some(value) => {
                buffer.write(true);
                buffer.write(value);
            },
            None => {
                buffer.write(false);
            }
        }
    }
}

impl<T: Serializable + Clone> Serializable for &[T] {
    fn serialize(self, buffer: &mut Buffer) {
        for item in self {
            buffer.write(item.clone())
        }
    }
}