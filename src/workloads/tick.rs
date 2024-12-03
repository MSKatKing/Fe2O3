use std::io::{ErrorKind, Read, Write};
use std::ops::Deref;
use std::time::Instant;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use shipyard::{AllStoragesViewMut, EntityId, Get, IntoIter, IntoWithId, ViewMut};
use packet::{Buffer, VarInt};
use crate::game::entities::player::Player;
use crate::networking::player::{Connection, PlayerState};
use crate::networking::server::NetworkingHandler;
use crate::safe_borrow_unique;
use crate::networking::packet::{add_outgoing_packet, add_packet, Bus, OutgoingPacket, Packet};
use crate::networking::packet::play::SynchronizePlayerPosition;

pub fn handle_networking_connection(mut storages: AllStoragesViewMut) {
    while let Ok((stream, addr)) = safe_borrow_unique!(storages, NetworkingHandler, accept) {
        let id = storages.add_entity(());

        tracing::debug!("Successful connection from {addr:?}");

        stream.set_nonblocking(true)
            .expect("Failed to set client stream to non-blocking");
        storages.add_component(id, Connection::new(stream, id));
    }
}

fn process_player_packets(entities_to_remove: &mut Vec<EntityId>, packets_to_add: &mut Vec<(EntityId, PlayerState, Packet)>, unknown_packets_to_add: &mut Vec<(EntityId, Buffer)>, id: EntityId, player: &mut Connection) {
    'read: loop {
        let mut buffer = [0u8;512];
        match player.stream.read(&mut buffer) {
            Ok(0) => {
                if !player.username.is_empty() {
                    tracing::info!("Player {} disconnected. (Socket closed)", player.username);
                } else {
                    tracing::debug!("Player disconnected.");
                }
                entities_to_remove.push(player.id);
                break 'read;
            }
            Ok(len) => {
                let mut buffer = Buffer::from(&buffer[..]);
                loop {
                    let packet = Packet::try_from((&mut buffer, player.compression_settings));

                    if let Err(msg) = packet {
                        tracing::error!("{msg}");
                        break;
                    }

                    let packet = packet.unwrap();

                    let id = packet.id;

                    packets_to_add.push((player.id.clone(), player.state.clone(), packet));

                    if buffer.cursor >= len {
                        break;
                    }

                    if (player.state == PlayerState::HANDSHAKE && id == 0x00) || (player.state == PlayerState::LOGIN && id == 0x03) {
                        unknown_packets_to_add.push((player.id.clone(), Buffer::from(&buffer[buffer.cursor..len])));
                        break 'read;
                    }
                }
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::WouldBlock => { },
                    ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted | ErrorKind::ConnectionRefused => {
                        if !player.username.is_empty() {
                            tracing::info!("Player {} disconnected. (Connection reset)", player.username);
                        }
                        entities_to_remove.push(id);
                    },
                    _ => {
                        tracing::error!("{e:?}");
                        entities_to_remove.push(id);
                    }
                }
                break 'read;
            }
        }
    }
}

pub fn handle_networking_incoming(mut storages: AllStoragesViewMut) {
    let mut players = storages.borrow::<ViewMut<Connection>>()
        .expect("Failed to borrow from storages");

    let mut entities_to_remove = Vec::new();
    let mut packets_to_add = Vec::new();
    let mut unknown_packets_to_add = Vec::new();

    for (id, player) in (&mut players).iter().with_id() {
        process_player_packets(&mut entities_to_remove, &mut packets_to_add, &mut unknown_packets_to_add, id, player);
    }

    drop(players);

    let mut players = storages.borrow::<ViewMut<Player>>()
        .expect("Failed to borrow from storages");

    for (id, player) in (&mut players).iter().with_id() {
        process_player_packets(&mut entities_to_remove, &mut packets_to_add, &mut unknown_packets_to_add, id, &mut player.connection);
    }

    drop(players);

    for (id, buffer) in unknown_packets_to_add {
        storages.add_component(id, buffer);
    }

    for (id, state, packet) in packets_to_add {
        add_packet(&mut storages, id, state, packet);
    }

    for id in entities_to_remove {
        storages.delete_entity(id);
    }
}

pub fn handle_networking_outgoing(mut vm_outgoing: ViewMut<Bus<OutgoingPacket>>, mut vm_connections: ViewMut<Connection>, mut vm_players: ViewMut<Player>) {
    for (id, packet) in (&mut vm_outgoing).iter().with_id() {
        let mut borrowed_connection = (&mut vm_connections).get(id);
        let mut borrowed_player = (&mut vm_players).get(id);

        let mut player: &mut Connection;

        if let Ok(ref mut connection) = &mut borrowed_connection {
            player = connection;
        } else if let Ok(connection) = &mut borrowed_player {
            player = &mut connection.connection;
        } else {
            tracing::error!("Failed to borrow either player or connection.");
            continue;
        }

        for p in packet.drain(..) {
            if player.compression_settings.is_some() && !(p.id == 0x03 && player.state == PlayerState::LOGIN) {
                if p.cursor >= player.compression_settings.unwrap() as usize {
                    let mut compressed = Buffer::new();
                    compressed.write(VarInt(p.id as i32));
                    compressed.write(p.deref().buffer.as_slice());

                    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                    encoder.write_all(compressed.buffer.as_slice()).unwrap();

                    let mut out = Buffer::new();
                    out.write(VarInt(compressed.cursor as i32));
                    out.write(encoder.finish().unwrap().as_slice());

                    let mut b = Buffer::new();
                    b.write(out.buffer);

                    player.stream.write_all(&b.buffer)
                        .expect("Failed to write to stream");
                } else {
                    let mut out = Buffer::new();
                    out.write(VarInt(0));
                    out.write(VarInt(p.id as i32));
                    out.write(p.deref().buffer.as_slice());

                    let mut b = Buffer::new();
                    b.write(out.buffer);

                    player.stream.write_all(&b.buffer)
                        .expect("Failed to write to stream");
                }
            } else {
                let mut new_buffer = Buffer::new();
                new_buffer.write(VarInt(p.id as i32));

                let length = new_buffer.cursor + p.buffer.cursor;
                let mut new_buffer = Buffer::new();
                new_buffer.write(VarInt(length as i32));
                new_buffer.write(VarInt(p.id as i32));
                new_buffer.buffer.write_all(&p.buffer.buffer[..p.buffer.cursor])
                    .unwrap();
                new_buffer.cursor += p.buffer.cursor;

                player.stream.write_all(&new_buffer.buffer)
                    .expect("Failed to write to stream");
            }
        }

        player.stream.flush().expect("Failed to flush stream");
    }
}

pub fn handle_unsent_player_packets(mut vm_players: ViewMut<Player>) {
    for player_ in (&mut vm_players).iter() {
        let player = &mut player_.connection;

        for p in player_.unprocessed_packets.drain(..) {
            if player.compression_settings.is_some() && !(p.id == 0x03 && player.state == PlayerState::LOGIN) {
                if p.cursor >= player.compression_settings.unwrap() as usize {
                    let mut compressed = Buffer::new();
                    compressed.write(VarInt(p.id as i32));
                    compressed.write(p.deref().buffer.as_slice());

                    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                    encoder.write_all(compressed.buffer.as_slice()).unwrap();

                    let mut out = Buffer::new();
                    out.write(VarInt(compressed.cursor as i32));
                    out.write(encoder.finish().unwrap().as_slice());

                    let mut b = Buffer::new();
                    b.write(out.buffer);

                    player.stream.write_all(&b.buffer)
                        .expect("Failed to write to stream");
                } else {
                    let mut out = Buffer::new();
                    out.write(VarInt(0));
                    out.write(VarInt(p.id as i32));
                    out.write(p.deref().buffer.as_slice());

                    let mut b = Buffer::new();
                    b.write(out.buffer);

                    player.stream.write_all(&b.buffer)
                        .expect("Failed to write to stream");
                }
            } else {
                let mut new_buffer = Buffer::new();
                new_buffer.write(VarInt(p.id as i32));

                let length = new_buffer.cursor + p.buffer.cursor;
                let mut new_buffer = Buffer::new();
                new_buffer.write(VarInt(length as i32));
                new_buffer.write(VarInt(p.id as i32));
                new_buffer.buffer.write_all(&p.buffer.buffer[..p.buffer.cursor])
                    .unwrap();
                new_buffer.cursor += p.buffer.cursor;

                player.stream.write_all(&new_buffer.buffer)
                    .expect("Failed to write to stream");
            }
        }

        player.stream.flush().expect("Failed to flush player stream");
    }
}

pub fn handle_teleport_requests(mut vm_outgoing: ViewMut<Bus<OutgoingPacket>>, mut vm_players: ViewMut<Player>) {
    for (id, player) in (&mut vm_players).iter().with_id() {
        for (tid, location, unsent) in &mut player.teleport_requests {
            if *unsent {
                *unsent = false;

                tracing::debug!("Sending player synchronize packet");

                add_outgoing_packet(&mut vm_outgoing, id, SynchronizePlayerPosition::new(location, tid.clone()))
            }
        }
    }
}

pub fn handle_keep_alives(mut vm_players: ViewMut<Player>, mut vm_outgoing: ViewMut<Bus<OutgoingPacket>>) {
    for player in (&mut vm_players).iter() {
        if player.last_keep_alive.elapsed().as_millis() > 5000 {
            player.last_keep_alive = Instant::now();

            player.send_keep_alive(&mut vm_outgoing);
        }
    }
}