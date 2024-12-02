use shipyard::{Component, Get, ViewMut};
use fe2o3_nbt::NBT;
use packet::{Identifier, VarInt};
use packet_proc::{outgoing, packet, Deserializable, Serializable};
use crate::game::entities::DeathLocation;
use crate::game::entities::player::Player;
use crate::game::Location;
use crate::networking::packet::{add_outgoing_packet, Bus, OutgoingPacket, PacketHandler};
use crate::networking::player::Connection;

#[packet(0x00)]
pub struct ConfirmTeleportation {
    teleport_id: TeleportID
}

#[packet(0x1A)]
pub struct SetPlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}

impl PacketHandler for SetPlayerPosition {
    type Included<'a> = ViewMut<'a, Player>;

    fn handler<'a>(mut vm_self: ViewMut<Self>, _: ViewMut<'a, Bus<OutgoingPacket>>, _: ViewMut<'a, Connection>, mut vm_players: Self::Included<'a>) {
        for (id, packet) in vm_self.drain().with_id() {
            let mut player = (&mut vm_players).get(id)
                .expect("Player should exist");

            let mut location = Location::from(packet);
            location.yaw = player.yaw();
            location.pitch = player.pitch();

            player.move_absolute(location);
        }
    }
}

#[packet(0x21)]
pub struct PingRequest {
    payload: u64
}

impl PacketHandler for PingRequest {
    type Included<'a> = ();

    fn handler<'a>(mut vm_self: ViewMut<Self>, mut vm_outgoing: ViewMut<'a, Bus<OutgoingPacket>>, _: ViewMut<'a, Connection>, _: Self::Included<'a>) {
        for (id, ping) in vm_self.drain().with_id() {
            add_outgoing_packet(&mut vm_outgoing, id, PingResponse { payload: ping.payload });
        }
    }
}

#[packet(0x22)]
#[outgoing]
pub struct GameEvent {
    pub event: u8,
    pub value: f32
}

#[packet(0x27)]
#[outgoing]
pub struct ChunkDataAndUpdateLight {
    pub x: i32,
    pub z: i32,
    pub heightmaps: NBT,
    pub data: Vec<u8>,
    pub block_entities: Vec<u8>,
    pub sky_light_mask: Vec<u8>,
    pub block_light_mask: Vec<u8>,
    pub empty_sky_light_mask: Vec<u8>,
    pub empty_block_light_mask: Vec<u8>,
    pub sky_light_array: Vec<u8>,
    pub block_light_array: Vec<u8>
}

#[packet(0x2B)]
#[outgoing]
pub struct PlayLogin {
    pub e_id: i32,
    pub is_hardcore: bool,
    pub dimensions: Vec<Identifier>, // for now always 0
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawns: bool,
    pub limited_crafting: bool,
    pub dimension_type: VarInt,
    pub dimension_name: Identifier,
    pub seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<DeathLocation>,
    pub portal_cooldown: VarInt,
    pub enforces_secure_chat: bool
}

#[packet(0x36)]
#[outgoing]
pub struct PingResponse {
    payload: u64
}

#[packet(0x38)]
#[outgoing]
pub struct PlayerAbilities {
    pub abilities: u8,
    pub flying_speed: f32,
    pub fov_modifier: f32
}

impl Default for PlayerAbilities {
    fn default() -> Self {
        Self {
            abilities: 0x01 | 0x02 | 0x04 | 0x08,
            flying_speed: 0.05,
            fov_modifier: 0.1
        }
    }
}

#[packet(0x3E)]
#[outgoing]
pub struct PlayerInfoUpdate {
    pub actions: u8,
    pub players: Vec<PlayerProperties>
}

#[derive(Serializable, Deserializable)]
struct PlayerProperties {
    name: String,
    number_of_properties: VarInt,
}

impl PlayerInfoUpdate {
    pub fn empty() -> Self {
        Self {
            actions: 0,
            players: vec![]
        }
    }

    pub fn for_current_player(player: &Player) -> Self {
        Self {
            actions: 0x01,
            players: vec![PlayerProperties {
                name: player.name().clone(),
                number_of_properties: VarInt(0),
            }]
        }
    }
}

#[packet(0x40)]
#[outgoing]
pub struct SynchronizePlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    pub teleport_id: TeleportID

}

#[derive(Serializable, Deserializable, Component, Clone)]
pub struct TeleportID {
    pub id: VarInt
}

impl SynchronizePlayerPosition {
    pub fn new(location: &Location, teleport_id: TeleportID) -> Self {
        Self {
            x: location.x,
            y: location.y,
            z: location.z,
            yaw: location.yaw,
            pitch: location.pitch,
            flags: 0,
            teleport_id,
        }
    }
}

#[packet(0x54)]
#[outgoing]
pub struct SetCenterChunk {
    pub x: VarInt,
    pub z: VarInt
}