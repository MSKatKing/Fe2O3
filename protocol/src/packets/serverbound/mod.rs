mod handshake;

use shipyard::{AddComponent, IntoIter, IntoWithId, Remove, View, ViewMut};
use data::network::{Connection, ConnectionState, ReadBytes};
use data::queue::Queue;
use crate::data::{PacketData, VarInt};
use crate::packets::Packet;
use crate::packets::serverbound::handshake::Handshake;

pub fn deserialize_serverbound_packets(mut vm_connection_state: ViewMut<ConnectionState>, v_connection: View<Connection>, mut vm_read_bytes: ViewMut<ReadBytes>) {
    for (id, (connection_state, _)) in (&mut vm_connection_state, &v_connection).iter().with_id() {
        let Some(bytes) = vm_read_bytes.remove(id) else { continue; };
        let mut bytes = bytes.0;

        while bytes.len() > 0 {
            let mut queue = bytes.into();
            let len = VarInt::deserialize(&mut queue).unwrap().0;
            let queue: Vec<u8> = queue.into();

            let mut queue: Queue = queue.into();
            let id = VarInt::deserialize(&mut queue).unwrap().0;

            match connection_state {
                ConnectionState::Handshake => {
                    if let Some(data) = Handshake::deserialize(&mut queue) {

                    }
                }
                ConnectionState::Status => {}
                ConnectionState::Login => {}
                ConnectionState::Transfer => {}
                ConnectionState::Configure => {}
                ConnectionState::Play => {}
            }

            bytes = queue.into();
            bytes = bytes[len..];
        }
    }
}