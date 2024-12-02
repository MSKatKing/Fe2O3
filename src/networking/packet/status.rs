use shipyard::{IntoIter, ViewMut};
use packet_proc::{outgoing, packet};
use text_component::{Component, TextColor, TextStyle};
use crate::networking::packet::{add_outgoing_packet, Bus, OutgoingPacket, PacketHandler};
use crate::networking::player::{Connection, PlayerState};

pub const VERSION_NAME: &str = "1.21.1";
pub const PROTOCOL_VERSION: usize = 767;

#[packet(0)]
pub struct StatusRequest;

impl PacketHandler for StatusRequest {
    type Included<'a> = ();

    fn handler<'a>(mut vm_self: ViewMut<Self>, mut vm_outgoing: ViewMut<'a, Bus<OutgoingPacket>>, vm_players: ViewMut<'a, Connection>, _: Self::Included<'a>) {
        let mut current_players = 0usize;
        for p in vm_players.iter() {
            if p.state == PlayerState::PLAY {
                current_players += 1;
            }
        }

        for (id, _) in vm_self.drain().with_id() {
            let mut description = Component::new("Hello, world!");
            description.color(TextColor::Cyan).style(TextStyle::Bold).append(Component::new_with_color(" (from Rust)", TextColor::DarkGray));

            let status = StatusResponse::new(100, current_players, description, false);

            add_outgoing_packet(&mut vm_outgoing, id, status);
        }
    }
}

#[packet(1)]
pub struct StatusPingRequest {
    pub timestamp: u64
}

impl PacketHandler for StatusPingRequest {
    type Included<'a> = ();

    fn handler<'a>(mut vm_self: ViewMut<Self>, mut vm_outgoing: ViewMut<'a, Bus<OutgoingPacket>>, _: ViewMut<'a, Connection>, _: Self::Included<'a>) {
        for (id, request) in vm_self.drain().with_id() {
            let out = StatusPongResponse {
                timestamp: request.timestamp
            };

            add_outgoing_packet(&mut vm_outgoing, id, out);
        }
    }
}

#[packet(0)]
#[outgoing]
pub struct StatusResponse {
    pub response: String
}

impl StatusResponse {
    pub fn new(max_players: usize, curr_players: usize, description: Component, enforces_secure_chat: bool) -> Self {
        tracing::debug!(include_str!("status_response_format.text"), VERSION_NAME, PROTOCOL_VERSION, max_players, curr_players, description.to_string(), enforces_secure_chat);
        Self {
            response: format!(include_str!("status_response_format.text"), VERSION_NAME, PROTOCOL_VERSION, max_players, curr_players, description.to_string(), enforces_secure_chat)
        }
    }
}

#[packet(1)]
#[outgoing]
pub struct StatusPongResponse {
    pub timestamp: u64
}