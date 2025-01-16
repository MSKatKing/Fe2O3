use std::collections::HashSet;
use std::random::random;
use std::time::Instant;
use shipyard::{Component, EntityId, ViewMut};
use uuid::Uuid;
use fe2o3_nbt::NBT;
use packet::{Buffer, Deserializable, Packet, Serializable, VarInt};
use crate::game::entities::{Disconnectable, Entity, Nameable, Positionable};
use crate::game::Location;
use crate::game::world::chunk::{Chunk, ChunkPosition};
use crate::networking::packet::{add_outgoing_packet, Bus, OutgoingPacket};
use crate::networking::packet::configuration::{ConfigurationDisconnect, ConfigurationPing};
use crate::networking::packet::login::LoginDisconnect;
use crate::networking::packet::play::{ChunkDataAndUpdateLight, PlayDisconnect, PlayPing, SetCenterChunk, TeleportID, UnloadChunk};
use crate::networking::player::{Connection, PlayerState};

#[derive(Component)]
pub struct Player {
    username: String,
    display_name: String,
    uuid: Uuid,
    brand: String,

    pub e_id: EntityId,

    view_distance: u8,
    locale: String,
    main_hand: MainHand,

    game_mode: GameMode,

    location: Location,
    chunk_x: i32,
    chunk_z: i32,

    pub connection: Connection,

    pub teleport_requests: Vec<(TeleportID, Location, bool)>,

    // TODO: change this eventually to a separate struct so that it can be an option for a timeout option on the config
    pub last_keep_alive: Instant,
    pub last_keep_alive_id: i32,

    pub unprocessed_packets: Bus<OutgoingPacket>
}

impl Player {
    pub fn new(username: String, uuid: Uuid, locale: String, main_hand: MainHand, e_id: EntityId, connection: Connection) -> Self {
        Self {
            username: username.clone(),
            display_name: username,
            uuid,
            e_id,
            view_distance: 0,
            locale,
            main_hand,
            game_mode: GameMode::default(),
            location: Location::new(0.0, 0.0, 0.0),
            chunk_x: 0,
            chunk_z: 0,
            brand: "".to_string(),
            teleport_requests: vec![],
            last_keep_alive: Instant::now(),
            last_keep_alive_id: 0,
            connection,
            unprocessed_packets: Bus::default()
        }
    }

    pub fn update_view_distance(&mut self, new_view_distance: u8) {
        self.view_distance = new_view_distance;
    }

    pub fn actual_view_distance(&self, server_view_distance: u8) -> u8 {
        self.view_distance.min(server_view_distance)
    }

    pub fn update_brand(&mut self, brand: String) {
        self.brand = brand;
    }

    pub fn brand(&self) -> &String {
        &self.brand
    }

    pub fn teleport_acknowledge(&mut self, teleport_id: TeleportID) {
        if let Some((id, _)) = self.teleport_requests.iter().enumerate().find(|(_, (id, _, unsent))| id.id == teleport_id.id && !unsent) {
            let (_, new_location, _) = self.teleport_requests.remove(id);
            self.location = &self.location + &new_location;
        } else {
            tracing::warn!("Player {} received a teleport acknowledgement for a teleport request that was already fulfilled or was unsent.", self.username);
        }
    }

    fn add_packet<T: Packet>(&mut self, packet: T) {
        self.unprocessed_packets.push(OutgoingPacket {
            id: packet.get_id(),
            buffer: packet.into_buffer()
        });
    }

    pub fn send_keep_alive(&mut self, vm_outgoing: &mut ViewMut<Bus<OutgoingPacket>>) {
        let id = random();

        self.last_keep_alive_id = self.last_keep_alive_id;
        self.last_keep_alive = Instant::now();

        match self.connection.state {
            PlayerState::CONFIGURATION => {
                add_outgoing_packet(vm_outgoing, self.e_id, ConfigurationPing { id })
            },
            PlayerState::PLAY => {
                add_outgoing_packet(vm_outgoing, self.e_id, PlayPing { id })
            }
            _ => {}
        }
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
    }
}

impl Positionable for Player {
    fn position(&self) -> &Location {
        &self.location
    }

    fn move_relative(&mut self, relative: Location) {
        if (relative.magnitude() >= 0.25) && !(self.game_mode == GameMode::Spectator || self.game_mode == GameMode::Creative) {
            tracing::warn!("Player {} ({}) moved too quickly! (x: {}, y: {}, z: {})", self.username, self.uuid, relative.x(), relative.y(), relative.z());
            self.teleport(relative);
        } else {
            if relative.x() as i32 / 16 != self.chunk_x ||
                relative.z() as i32 / 16 != self.chunk_z {
                let old_chunk_x = self.chunk_x;
                let old_chunk_z = self.chunk_z;

                self.chunk_x = relative.x() as i32 / 16;
                self.chunk_z = relative.z() as i32 / 16;

                self.add_packet(SetCenterChunk {
                    x: VarInt(self.chunk_x),
                    z: VarInt(self.chunk_z)
                });

                let loaded_chunks = ((old_chunk_x - self.view_distance as i32)..=(old_chunk_x + self.view_distance as i32))
                    .into_iter()
                    .flat_map(|x| ((old_chunk_z - self.view_distance as i32)..=(old_chunk_z + self.view_distance as i32)).map(move |z| (x, z)))
                    .collect::<Vec<_>>();
                let new_chunks = ((self.chunk_x - self.view_distance as i32)..=(self.chunk_x + self.view_distance as i32))
                    .into_iter()
                    .flat_map(|x| ((self.chunk_z - self.view_distance as i32)..=(self.chunk_z + self.view_distance as i32)).map(move |z| (x, z)))
                    .collect::<Vec<_>>();

                let unload_chunks = new_chunks
                    .iter()
                    .collect::<HashSet<_>>();

                let unload_chunks = loaded_chunks
                    .iter()
                    .filter(|v| unload_chunks.contains(v))
                    .collect::<Vec<_>>();

                let load_chunks = loaded_chunks
                    .iter()
                    .collect::<HashSet<_>>();

                let load_chunks = new_chunks
                    .iter()
                    .filter(|v| load_chunks.contains(v))
                    .collect::<Vec<_>>();

                tracing::debug!("Loaded chunks: {:?}", load_chunks);
                tracing::debug!("Unloaded chunks: {:?}", unload_chunks);

                for (chunk_x, chunk_z) in unload_chunks {
                    self.add_packet(UnloadChunk {
                        chunk_x: *chunk_x,
                        chunk_z: *chunk_z,
                    })
                }

                let chunk = Chunk::flat_generation();

                for (chunk_x, chunk_z) in load_chunks {
                    self.add_packet::<ChunkDataAndUpdateLight>((&ChunkPosition { x: *chunk_x, z: *chunk_z }, &chunk).into())
                }
            }

            self.location = relative;
        }
    }

    fn move_absolute(&mut self, absolute: Location) {
        self.move_relative(absolute)
    }

    fn teleport(&mut self, location: Location) {
        self.teleport_requests.push((TeleportID { id: VarInt(random()) }, location, true));
    }
}
impl Nameable for Player {
    fn name(&self) -> &str {
        self.username.as_str()
    }

    fn display_name(&self) -> &str {
        self.display_name.as_str()
    }

    fn set_display_name(&mut self, display_name: impl Into<String>) {
        self.display_name = display_name.into()
    }
}
impl Disconnectable for Player {
    fn kick(&mut self, message: impl Into<text_component::Component>) {
        let message = message.into();
        let component: NBT = message.into();

        match self.connection.state {
            PlayerState::LOGIN => {
                self.add_packet(LoginDisconnect { component })
            },
            PlayerState::CONFIGURATION => {
                self.add_packet(ConfigurationDisconnect { component })
            },
            PlayerState::PLAY => {
                self.add_packet(PlayDisconnect { component })
            }
            _ => {
                tracing::warn!("Attempted to kick player {} but the connection was in a state that doesn't allow kick packets.", self.name())
            }
        }
    }
}

impl Entity for Player {

}

#[repr(i8)]
#[derive(Default, PartialEq)]
pub enum GameMode {
    Undefined = -1,
    #[default]
    Survival,
    Creative,
    Adventure,
    Spectator
}

impl Serializable for GameMode {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(VarInt(self as i8 as i32));
    }
}

impl Deserializable for GameMode {
    fn deserialize(buffer: &mut Buffer) -> Self {
        let int: i8 = buffer.read::<VarInt>().0 as i8;
        match int {
            -1 => Self::Undefined,
            0 => Self::Survival,
            1 => Self::Creative,
            2 => Self::Adventure,
            3 => Self::Spectator,
            _ => Self::default()
        }
    }
}

#[repr(u8)]
pub enum MainHand {
    Left,
    Right
}

impl Deserializable for MainHand {
    fn deserialize(buffer: &mut Buffer) -> Self {
        match buffer.read::<VarInt>().0 {
            0 => Self::Left,
            _ => Self::Right,
        }
    }
}

impl Serializable for MainHand {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(VarInt(self as u8 as i32));
    }
}