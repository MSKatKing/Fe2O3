use packet_proc::state_changing;
use shipyard::{Get, ViewMut};
use packet::VarInt;
use packet_proc::{packet, packet_handler};
use crate::networking::player::{Connection, PlayerState};

#[packet(id = 0)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub address: String,
    pub port: u16,
    pub next_state: PlayerState
}

#[packet_handler(packet = Handshake, state_changing)]
fn handler(mut vm_self: ViewMut<Handshake>, mut vm_players: ViewMut<Connection>) {
    for (id, handshake) in vm_self.drain().with_id() {
        let mut player = (&mut vm_players).get(id)
            .expect("Player did not have a Connection");

        if player.state != PlayerState::HANDSHAKE {
            tracing::error!("Received a handshake but the player was not in the Handshake state.");
            continue;
        }

        player.state = handshake.next_state;
    }
}