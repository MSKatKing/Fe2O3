use shipyard::Unique;
use packet::Identifier;
use crate::game::entities::EntityManager;
use crate::game::world::chunk::ChunkManager;

pub mod chunk;
mod heightmap;
mod block;

#[derive(Unique)]
pub struct WorldHandler {
    worlds: Vec<World>
}

impl WorldHandler {
    pub fn new() -> Self {
        Self {
            worlds: Vec::new()
        }
    }
}

pub struct World {
    chunk_manager: ChunkManager,

    identifier: Identifier,

    entities_manager: EntityManager
}