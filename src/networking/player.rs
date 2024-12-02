use std::net::TcpStream;
use shipyard::{Component, EntityId};
use uuid::Uuid;
use packet::{Buffer, Deserializable, Serializable, VarInt};
use crate::networking::chat::ChatSettings;

#[derive(Component)]
pub struct Connection {
    pub stream: TcpStream,
    pub id: EntityId,

    pub state: PlayerState,
    pub username: String,
    pub uuid: Uuid,

    pub display_in_server_list: bool,
    pub chat_settings: ChatSettings,

    pub text_filtering: bool,

    pub compression_settings: Option<u32>
}

impl Connection {
    pub fn new(stream: TcpStream, id: EntityId) -> Self {
        Self {
            stream,
            id,
            state: PlayerState::HANDSHAKE,
            username: String::new(),
            uuid: Default::default(),
            display_in_server_list: false,
            chat_settings: ChatSettings::default(),
            text_filtering: true,
            compression_settings: None
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum PlayerState {
    HANDSHAKE,
    STATUS,
    LOGIN,
    TRANSFER,
    CONFIGURATION,
    PLAY
}

impl Serializable for PlayerState {
    fn serialize(self, buffer: &mut Buffer) {
        buffer.write(VarInt(self as u8 as i32));
    }
}

impl Deserializable for PlayerState {
    fn deserialize(buffer: &mut Buffer) -> Self {
        let value: VarInt = buffer.read();
        PlayerState::from(value.0 as u8)
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::HANDSHAKE
    }
}

impl From<u8> for PlayerState {
    fn from(value: u8) -> Self {
        match value {
            0 => PlayerState::HANDSHAKE,
            1 => PlayerState::STATUS,
            2 => PlayerState::LOGIN,
            3 => PlayerState::TRANSFER,
            4 => PlayerState::CONFIGURATION,
            5 => PlayerState::PLAY,
            _ => PlayerState::HANDSHAKE
        }
    }
}