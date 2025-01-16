use shipyard::{Get, ViewMut};
use uuid::Uuid;
use fe2o3_nbt::NBT;
use packet::{ByteArrayInferredLength, Identifier, VarInt};
use packet_proc::{outgoing, packet, packet_handler, state_changing};
use crate::networking::packet::{add_outgoing_packet, Bus, OutgoingPacket};
use crate::networking::packet::configuration::RegistryData;
use crate::networking::player::{Connection, PlayerState};

#[packet(id = 0)]
pub struct LoginStart {
    pub name: String,
    pub uuid: Uuid
}

#[packet_handler(packet = LoginStart)]
fn handler(mut vm_self: ViewMut<LoginStart>, mut vm_outgoing: ViewMut<Bus<OutgoingPacket>>, mut vm_players: ViewMut<Connection>) {
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

#[packet(id = 1)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub shared_token: Vec<u8>,
}

#[packet(id = 2)]
pub struct LoginPluginResponse {
    pub message_id: VarInt,
    pub successful: bool,
    pub data: ByteArrayInferredLength
}

#[packet(id = 3)]
pub struct LoginAcknowledged;

#[packet_handler(packet = LoginAcknowledged, state_changing)]
fn handler(mut vm_self: ViewMut<LoginAcknowledged>, mut vm_outgoing: ViewMut<Bus<OutgoingPacket>>, mut vm_players: ViewMut<Connection>) {
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

#[packet(id = 4)]
pub struct LoginCookieResponse {
    pub key: Identifier,
    pub payload: Option<Vec<u8>>
}

#[packet(id = 0x00)]
#[outgoing]
pub struct LoginDisconnect {
    pub component: NBT
}

#[packet(id = 0x01)]
#[outgoing]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
    pub should_auth: bool
}

#[packet(id = 0x02)]
#[outgoing]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<u8>,
    pub strict_error_handling: bool
}

#[packet(id = 0x03)]
#[outgoing]
pub struct SetCompression {
    pub threshold: VarInt
}

#[packet(id = 0x04)]
#[outgoing]
pub struct PluginRequest {
    pub id: VarInt,
    pub channel: Identifier,
    pub data: ByteArrayInferredLength
}

#[packet(id = 0x05)]
#[outgoing]
pub struct CookieRequest {
    pub key: Identifier
}