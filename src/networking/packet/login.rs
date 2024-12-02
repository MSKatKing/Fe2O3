use shipyard::{Get, ViewMut};
use uuid::Uuid;
use packet::{ByteArrayInferredLength, Identifier, VarInt};
use packet_proc::{outgoing, packet, state_changing};
use crate::networking::packet::{add_outgoing_packet, Bus, OutgoingPacket, PacketHandler};
use crate::networking::packet::configuration::RegistryData;
use crate::networking::player::{Connection, PlayerState};

#[packet(0)]
pub struct LoginStart {
    pub name: String,
    pub uuid: Uuid
}

impl PacketHandler for LoginStart {
    type Included<'a> = ();

    fn handler<'a>(mut vm_self: ViewMut<Self>, mut vm_outgoing: ViewMut<'a, Bus<OutgoingPacket>>, mut vm_players: ViewMut<'a, Connection>, _: Self::Included<'a>) {
        for (id, login_start) in vm_self.drain().with_id() {
            let username = login_start.name;
            let uuid = login_start.uuid;

            let mut player = (&mut vm_players).get(id)
                .expect("PlayerConnection should exist");

            player.username = username.clone();
            player.uuid = uuid;

            tracing::info!("UUID of player {} is {}.", username, uuid);

            player.compression_settings = Some(50);

            add_outgoing_packet(&mut vm_outgoing, id, SetCompression {
                threshold: VarInt(50),
            });
            add_outgoing_packet(&mut vm_outgoing, id, LoginSuccess {
                uuid,
                username,
                properties: vec![],
                strict_error_handling: false,
            });
        }
    }
}

#[packet(1)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub shared_token: Vec<u8>,
}

#[packet(2)]
pub struct LoginPluginResponse {
    pub message_id: VarInt,
    pub successful: bool,
    pub data: ByteArrayInferredLength
}

#[packet(3)]
pub struct LoginAcknowledged;

#[state_changing]
impl PacketHandler for LoginAcknowledged {
    type Included<'a> = ();

    fn handler<'a>(mut vm_self: ViewMut<Self>, mut vm_outgoing: ViewMut<'a, Bus<OutgoingPacket>>, mut vm_players: ViewMut<'a, Connection>, _: Self::Included<'a>) {
        for (id, _) in vm_self.drain().with_id() {
            let mut player = (&mut vm_players).get(id)
                .expect("PlayerConnection should exist");

            if player.state != PlayerState::LOGIN {
                tracing::error!("Received a LoginAcknowledged packet but the player was not in the Login state");
                continue;
            }

            player.state = PlayerState::CONFIGURATION;

            add_outgoing_packet(&mut vm_outgoing, id, RegistryData::new(json::parse(include_str!("../../resources/registry/dimension_type.json"))));
            add_outgoing_packet(&mut vm_outgoing, id, RegistryData::new(json::parse(include_str!("../../resources/registry/biomes.json"))));
            add_outgoing_packet(&mut vm_outgoing, id, RegistryData::new(json::parse(include_str!("../../resources/registry/wolf_variant.json"))));
            add_outgoing_packet(&mut vm_outgoing, id, RegistryData::new(json::parse(include_str!("../../resources/registry/painting_variant.json"))));
            add_outgoing_packet(&mut vm_outgoing, id, RegistryData::new(json::parse(include_str!("../../resources/registry/damage_type.json"))));
            add_outgoing_packet(&mut vm_outgoing, id, RegistryData::new(json::parse(include_str!("../../resources/registry/banner_pattern.json"))));
            add_outgoing_packet(&mut vm_outgoing, id, RegistryData::new(json::parse(include_str!("../../resources/registry/chat_type.json"))));
            add_outgoing_packet(&mut vm_outgoing, id, RegistryData::new(json::parse(include_str!("../../resources/registry/trim_material.json"))));
            add_outgoing_packet(&mut vm_outgoing, id, RegistryData::new(json::parse(include_str!("../../resources/registry/trim_pattern.json"))));
        }
    }
}

#[packet(4)]
pub struct LoginCookieResponse {
    pub key: Identifier,
    pub payload: Option<Vec<u8>>
}

#[packet(0x00)]
#[outgoing]
pub struct LoginDisconnect {

}

#[packet(0x01)]
#[outgoing]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
    pub should_auth: bool
}

#[packet(0x02)]
#[outgoing]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<u8>,
    pub strict_error_handling: bool
}

#[packet(0x03)]
#[outgoing]
pub struct SetCompression {
    pub threshold: VarInt
}

#[packet(0x04)]
#[outgoing]
pub struct PluginRequest {
    pub id: VarInt,
    pub channel: Identifier,
    pub data: ByteArrayInferredLength
}

#[packet(0x05)]
#[outgoing]
pub struct CookieRequest {
    pub key: Identifier
}