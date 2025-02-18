use shipyard::Component;
use data::queue::Queue;

pub mod specific_types;
mod serverbound;

pub trait Packet {
    fn serialize(self) -> Vec<u8>;
    fn deserialize(queue: &mut Queue) -> Option<Self>;
    fn id(&self) -> u8;
}