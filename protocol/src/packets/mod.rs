use data::queue::Queue;

pub mod specific_types;

pub trait Packet {
    fn serialize(self) -> Vec<u8>;
    fn deserialize(queue: &mut Queue) -> Option<Self>;
}