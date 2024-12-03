use std::random::random;
use std::time::Instant;
use shipyard::{Component, EntityId, ViewMut};
use uuid::Uuid;
use packet::{Buffer, Deserializable, Packet, Serializable, VarInt};
use crate::game::Location;
use crate::networking::packet::{add_outgoing_packet, Bus, OutgoingPacket};
use crate::networking::packet::configuration::{ConfigurationDisconnect, ConfigurationPing};
use crate::networking::packet::login::LoginDisconnect;
use crate::networking::packet::play::{PlayDisconnect, PlayPing, TeleportID};
use crate::networking::player::{Connection, PlayerState};

#[derive(Component)]
pub struct Player {
    username: String,
    uuid: Uuid,
    brand: String,

    pub e_id: EntityId,

    view_distance: u8,
    locale: String,
    main_hand: MainHand,

    game_mode: GameMode,

    location: Location,

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
            username,
            uuid,
            e_id,
            view_distance: 0,
            locale,
            main_hand,
            game_mode: GameMode::default(),
            location: Location::new(0.0, 0.0, 0.0),
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

    pub fn teleport(&mut self, location: Location) {
        self.teleport_requests.push((TeleportID { id: VarInt(random()) }, location, true));
    }

    pub fn teleport_acknowledge(&mut self, teleport_id: TeleportID) {
        if let Some((id, _)) = self.teleport_requests.iter().enumerate().find(|(_, (id, _, unsent))| id.id == teleport_id.id && !unsent) {
            let (_, new_location, _) = self.teleport_requests.remove(id);
            self.location = &self.location + &new_location;
        } else {
            tracing::warn!("Player {} received a teleport acknowledgement for a teleport request that was already fulfilled or was unsent.", self.username);
        }
    }

    pub fn move_relative(&mut self, new_location: Location) {
        if (new_location.magnitude() >= 0.25) && !(self.game_mode == GameMode::Spectator || self.game_mode == GameMode::Creative) {
            tracing::warn!("Player {} ({}) moved too quickly! (x: {}, y: {}, z: {})", self.username, self.uuid, new_location.x(), new_location.y(), new_location.z());
            self.teleport(new_location);
        } else {
            self.location = new_location;
        }
    }

    pub fn move_absolute(&mut self, new_location: Location) {
        self.move_relative(self.location.relative(&new_location))
    }

    pub fn yaw(&self) -> f32 {
        self.location.yaw()
    }

    pub fn pitch(&self) -> f32 {
        self.location.pitch()
    }

    pub fn name(&self) -> &String {
        &self.username
    }

    fn add_packet<T: Packet>(&mut self, packet: T) {
        self.unprocessed_packets.push(OutgoingPacket {
            id: packet.get_id(),
            buffer: packet.into_buffer()
        });
    }

    pub fn kick(&mut self, component: text_component::Component) {
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
            _ => {}
        }
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