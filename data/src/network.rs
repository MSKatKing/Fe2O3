use shipyard::Component;
use crate::queue::Queue;

#[derive(Component)]
pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Transfer,
    Configure,
    Play,
}

#[derive(Component)]
pub struct Connection {

}

#[derive(Component)]
pub struct ReadBytes(pub Vec<u8>);