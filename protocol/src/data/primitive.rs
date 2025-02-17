use uuid::NonNilUuid;
use data::queue::Queue;
use crate::data::PacketData;
use crate::data::variable::VarInt;

impl PacketData for bool {
    fn serialize(self) -> Vec<u8> {
        vec![self as u8]
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        Some(queue.pop::<u8>()? != 0)
    }
}
impl PacketData for i8 {
    fn serialize(self) -> Vec<u8> {
        vec![self as u8]
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        queue.pop::<i8>()
    }
}
impl PacketData for u8 {
    fn serialize(self) -> Vec<u8> {
        vec![self]
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        queue.pop()
    }
}
impl PacketData for i16 {
    fn serialize(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        queue.pop()
    }
}
impl PacketData for u16 {
    fn serialize(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        queue.pop()
    }
}
impl PacketData for i32 {
    fn serialize(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        queue.pop()
    }
}
impl PacketData for i64 {
    fn serialize(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        queue.pop()
    }
}
impl PacketData for f32 {
    fn serialize(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        queue.pop()
    }
}
impl PacketData for f64 {
    fn serialize(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        queue.pop()
    }
}

impl PacketData for String {
    fn serialize(self) -> Vec<u8> {
        let mut out = VarInt(self.len() as i32).serialize();

        out.extend(self.as_bytes());

        out
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        let len = VarInt::deserialize(queue)?;
        let mut bytes = Vec::with_capacity(len.0 as usize);
        for _ in 0..len.0 {
            bytes.push(queue.pop::<u8>()?);
        }

        String::from_utf8(bytes).ok()
    }
}

impl<T: PacketData> PacketData for Vec<T> {
    fn serialize(self) -> Vec<u8> {
        let mut out = VarInt(self.len() as _).serialize();
        for value in self {
            out.extend(value.serialize());
        }
        out
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        let len = VarInt::deserialize(queue)?;
        let mut out = Vec::with_capacity(len.0 as _);
        for _ in 0..len.0 {
            out.push(T::deserialize(queue)?);
        }

        Some(out)
    }
}