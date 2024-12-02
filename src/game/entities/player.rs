use std::random::random;
use shipyard::{Component, EntityId};
use uuid::Uuid;
use packet::{Buffer, Deserializable, Serializable, VarInt};
use crate::game::Location;
use crate::networking::packet::play::TeleportID;

#[derive(Component)]
pub struct Player {
    username: String,
    uuid: Uuid,
    brand: String,

    e_id: EntityId,

    view_distance: u8,
    locale: String,
    main_hand: MainHand,

    game_mode: GameMode,

    location: Location,

    pub teleport_requests: Vec<(TeleportID, Location, bool)>,
}

#[repr(i8)]
#[derive(Default)]
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

impl Player {
    pub fn new(username: String, uuid: Uuid, locale: String, main_hand: MainHand, e_id: EntityId) -> Self {
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
        if new_location.x.abs() >= 4.0 || new_location.y.abs() >= 4.0 || new_location.z.abs() >= 4.0 {
            tracing::warn!("Player {} ({}) moved too quickly! (x: {}, y: {}, z: {})", self.username, self.uuid, new_location.x, new_location.y, new_location.z);
            self.teleport(new_location);
        } else {
            self.location = new_location;
        }
    }

    pub fn move_absolute(&mut self, new_location: Location) {
        self.move_relative(self.location.relative(&new_location))
    }

    pub fn yaw(&self) -> f32 {
        self.location.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.location.pitch
    }

    pub fn name(&self) -> &String {
        &self.username
    }
}