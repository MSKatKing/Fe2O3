use packet::{Identifier, Position};
use packet_proc::{Deserializable, Serializable};

pub mod player;

#[derive(Serializable, Deserializable)]
pub struct DeathLocation {
    pub dimension: Identifier,
    pub position: Position
}

pub trait Entity: Send + Sync + 'static {
    fn position(&self) -> Position;
    fn update_position(&mut self);
}

pub struct EntityManager {
    entities: Vec<Box<dyn Entity>>
}