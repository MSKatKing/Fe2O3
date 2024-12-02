use json::JsonValue;
use uuid::Uuid;
use crate::buffer::Buffer;
use crate::serializable::VarInt;

pub trait Deserializable: Send + Sync + Sized {
    fn deserialize(buffer: &mut Buffer) -> Self;
}

impl Deserializable for VarInt {
    fn deserialize(buffer: &mut Buffer) -> Self {
        const SEGMENT_BIT: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut value = 0;
        let mut position = 0;
        let mut byte: u8;

        loop {
            byte = buffer.read_numeric();
            value |= ((byte & SEGMENT_BIT) as i32) << position;

            if (byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;
            if position >= 32 {
                return VarInt(0);
            }
        }

        VarInt(value)
    }
}

impl Deserializable for String {
    fn deserialize(buffer: &mut Buffer) -> Self {
        let length = buffer.read::<VarInt>().0 as usize;

        let mut bytes = Vec::with_capacity(length);
        bytes.resize(length, 0);
        for i in 0..length {
            bytes[i] = buffer.read::<u8>()
        }

        String::from_utf8(bytes)
            .unwrap()
    }
}

impl Deserializable for u8 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<u8>()
    }
}

impl Deserializable for u16 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<u16>()
    }
}

impl Deserializable for u32 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<u32>()
    }
}

impl Deserializable for u64 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<u64>()
    }
}

impl Deserializable for u128 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<u128>()
    }
}

impl Deserializable for usize {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<usize>()
    }
}

impl Deserializable for i8 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<i8>()
    }
}

impl Deserializable for i16 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<i16>()
    }
}

impl Deserializable for i32 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<i32>()
    }
}

impl Deserializable for i64 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<i64>()
    }
}

impl Deserializable for i128 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<i128>()
    }
}

impl Deserializable for isize {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<isize>()
    }
}

impl Deserializable for f32 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<f32>()
    }
}

impl Deserializable for f64 {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<f64>()
    }
}

impl Deserializable for bool {
    fn deserialize(buffer: &mut Buffer) -> Self {
        buffer.read_numeric::<u8>() != 0
    }
}

impl Deserializable for Uuid {
    fn deserialize(buffer: &mut Buffer) -> Self {
        Uuid::from_u128(buffer.read())
    }
}

impl Deserializable for JsonValue {
    fn deserialize(buffer: &mut Buffer) -> Self {
        json::parse(buffer.read::<String>().as_str()).unwrap()
    }
}

impl<T: Deserializable> Deserializable for Vec<T> {
    fn deserialize(buffer: &mut Buffer) -> Self {
        let len = buffer.read::<VarInt>().0 as usize;

        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            out.push(buffer.read::<T>());
        }

        out
    }
}

impl<T: Deserializable> Deserializable for Option<T> {
    fn deserialize(buffer: &mut Buffer) -> Self {
        let is_present = buffer.read::<bool>();

        if is_present {
            Some(buffer.read())
        } else {
            None
        }
    }
}

impl<T: Deserializable> Deserializable for &[T] {
    fn deserialize(_: &mut Buffer) -> Self {
        &[]
    }
}