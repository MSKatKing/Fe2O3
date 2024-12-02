mod serializable;
mod buffer;
mod deserializable;
mod types;

pub use buffer::Buffer;
pub use serializable::{VarInt, Serializable};
pub use deserializable::Deserializable;
pub use types::*;

pub trait Packet: Send + Sync + Sized {
    const ID: usize;

    fn into_buffer(self) -> Buffer;
    fn from_buffer(buffer: Buffer) -> Self;

    fn get_id(&self) -> usize {
        Self::ID
    }
}