use shipyard::Component;
use protocol_proc::packet;
use crate::data::VarInt;

#[derive(Component)]
#[packet(0x00)]
pub struct Handshake {
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    next_state: VarInt,
}