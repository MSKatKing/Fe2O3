use data::queue::Queue;
use crate::data::PacketData;

/// This Option<T> is prefixed with a boolean that tells if the Option is present or not. The majority of Options in the packets are not like this, and instead must be inferred from previous data.
impl<T: PacketData> PacketData for Option<T> {
    fn serialize(self) -> Vec<u8> {
        let mut out = self.is_some().serialize();
        if let Some(data) = self {
            out.extend(data.serialize());
        }
        
        out
    }

    fn deserialize(queue: &mut Queue) -> Option<Self>
    where
        Self: Sized,
    {
        let present = bool::deserialize(queue)?;
        if present {
            Some(T::deserialize(queue))
        } else {
            None
        }
    }
}