use shipyard::{Get, ViewMut};
use packet::VarInt;
use packet_proc::{packet, state_changing};
use crate::networking::packet::{Bus, OutgoingPacket, PacketHandler};
use crate::networking::player::{Connection, PlayerState};

#[packet(0)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub address: String,
    pub port: u16,
    pub next_state: PlayerState
}

#[state_changing]
impl PacketHandler for Handshake {
    type Included<'a> = ();

    fn handler<'a>(mut vm_self: ViewMut<Self>, _: ViewMut<'a, Bus<OutgoingPacket>>, mut vm_players: ViewMut<'a, Connection>, _: Self::Included<'a>) {
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
}