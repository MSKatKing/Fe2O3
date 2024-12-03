pub mod handshake;
pub mod status;
pub mod login;
pub mod configuration;
pub mod play;

use std::io::Read;
use std::ops::{Deref, DerefMut};
use flate2::read::ZlibDecoder;
use shipyard::{AllStoragesViewMut, Component, EntityId, Get, View, ViewMut};
use packet::{Buffer, VarInt};
use packet_proc::add_packet_fn;
use crate::networking::packet::handshake::*;
use crate::networking::packet::status::*;
use crate::networking::packet::login::*;
use crate::networking::packet::configuration::*;
use crate::networking::packet::play::*;
use crate::networking::player::Connection;

#[derive(Component, Default)]
pub struct OutgoingPacket {
    pub id: usize,
    pub buffer: Buffer
}

impl Deref for OutgoingPacket {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
impl DerefMut for OutgoingPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

pub struct Packet {
    pub length: usize,
    pub id: usize,
    pub data: Buffer
}

impl TryFrom<(&mut Buffer, Option<u32>)> for Packet {
    type Error = String;

    fn try_from((value, compression): (&mut Buffer, Option<u32>)) -> Result<Self, Self::Error> {
        let length = value.read::<VarInt>().0 as usize;

        if length == 0x7E {
            return Ok(Self {
                length: 1,
                id: 0x7E,
                data: Buffer::from(value)
            })
        }

        if compression.is_some() {
            let data_length_length = value.cursor;

            let data_length = value.read::<VarInt>().0 as usize;

            let data_length_length = value.cursor - data_length_length;

            let mut decompressed = Buffer::new();

            if data_length > 0 {
                let mut decoder = ZlibDecoder::new(&value.buffer[value.cursor..(value.cursor + length - data_length_length)]);

                value.cursor += length - data_length_length;

                let mut d = Vec::new();
                let bytes = decoder.read_to_end(&mut d);
                if let Err(err) = bytes {
                    return Err(format!("Failed to decompress packet: \"{}\"", err.to_string()));
                }

                decompressed.write(d.as_slice());
            } else {
                decompressed.buffer.append(&mut value.buffer[value.cursor..(value.cursor + length - data_length_length)].to_vec());
                value.cursor += length - data_length_length;
            }

            let id = decompressed.read::<VarInt>().0 as usize;

            Ok(Self {
                length,
                id,
                data: Buffer::from(&decompressed.buffer[decompressed.cursor..])
            })
        } else {
            let id: VarInt = value.read();
            let id = id.0 as usize;

            let cursor = value.cursor;
            value.cursor += length - 1;

            Ok(Self {
                length,
                id,
                data: Buffer::from(&value.buffer[cursor..])
            })
        }
    }
}

#[derive(Component)]
pub struct Bus<T: Send + Sync + 'static>(Vec<T>);

impl<T: Send + Sync> Deref for Bus<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Send + Sync> DerefMut for Bus<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: Send + Sync> Default for Bus<T> {
    fn default() -> Self {
        Bus(Vec::new())
    }
}

add_packet_fn! {
    HANDSHAKE => "src::networking::packet::handshake",
    LOGIN => "src::networking::packet::login",
    STATUS => "src::networking::packet::status",
    CONFIGURATION => "src::networking::packet::configuration",
    PLAY => "src::networking::packet::play",
}

pub fn evaluate_unprocessed_packets(mut storages: AllStoragesViewMut) {
    let mut vm_unknown_packets = storages.borrow::<ViewMut<Buffer>>()
        .unwrap();

    let v_players = storages.borrow::<View<Connection>>()
        .unwrap();

    let mut packets_to_add = Vec::new();

    for (id, mut buffer) in vm_unknown_packets.drain().with_id() {
        let player = v_players.get(id)
            .expect("Player should exist");

        let len = buffer.len();
        loop {
            let packet = Packet::try_from((&mut buffer, player.compression_settings));

            if let Err(msg) = packet {
                tracing::error!("{msg}");
                break;
            }

            let packet = packet.unwrap();

            packets_to_add.push((player.id.clone(), player.state.clone(), packet));

            if buffer.cursor >= len {
                break;
            }
        }
    }

    drop(vm_unknown_packets);
    drop(v_players);

    for (id, state, packet) in packets_to_add {
        add_packet(&mut storages, id, state, packet)
    }
}

pub fn add_outgoing_packet<T: packet::Packet>(storages: &mut ViewMut<Bus<OutgoingPacket>>, id: EntityId, packet: T) {
    storages
        .get_or_insert_with(id, Default::default)
        .unwrap()
        .push(OutgoingPacket {
            id: packet.get_id(),
            buffer: packet.into_buffer()
        });
}