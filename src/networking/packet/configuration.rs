use json::JsonValue;
use shipyard::{AddComponent, Get, ViewMut};
use fe2o3_nbt::NBT;
use packet::{ByteArrayInferredLength, Identifier, VarInt};
use packet_proc::{outgoing, packet, state_changing, Deserializable, Serializable};
use crate::game::entities::player::{GameMode, MainHand, Player};
use crate::game::Location;
use crate::game::world::chunk::empty_heightmap;
use crate::networking::chat::ChatSettings;
use crate::networking::packet::{add_outgoing_packet, Bus, OutgoingPacket, PacketHandler};
use crate::networking::packet::play::{ChunkDataAndUpdateLight, GameEvent, PlayLogin, PlayerAbilities};
use crate::networking::player::{Connection, PlayerState};

#[packet(0)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: i8,
    pub chat_settings: ChatSettings,
    pub displayed_skin_parts: u8,
    pub main_hand: MainHand,
    pub text_filtering: bool,
    pub allow_server_listing: bool
}

impl PacketHandler for ClientInformation {
    type Included<'a> = ViewMut<'a, Player>;

    fn handler<'a>(mut vm_self: ViewMut<Self>, mut vm_outgoing: ViewMut<'a, Bus<OutgoingPacket>>, mut vm_connections: ViewMut<'a, Connection>, mut vm_players: Self::Included<'a>) {
        for (id, info) in vm_self.drain().with_id() {
            let mut player = (&mut vm_connections).get(id)
                .expect("PlayerConnection should exist");

            player.display_in_server_list = info.allow_server_listing;
            player.chat_settings = info.chat_settings;
            player.text_filtering = info.text_filtering;

            let mut player = Player::new(player.username.clone(), player.uuid, info.locale.clone(), info.main_hand, id);
            player.update_view_distance(info.view_distance as u8);

            vm_players.add_component_unchecked(id, player);

            add_outgoing_packet(&mut vm_outgoing, id, FinishConfiguration);
        }
    }
}

#[packet(2)]
pub struct PluginMessage {
    pub channel: Identifier,
    pub data: ByteArrayInferredLength
}

impl PacketHandler for PluginMessage {
    type Included<'a> = ViewMut<'a, Player>;

    fn handler<'a>(mut vm_self: ViewMut<Self>, _: ViewMut<'a, Bus<OutgoingPacket>>, _: ViewMut<'a, Connection>, mut vm_players: Self::Included<'a>) {
        for (id, msg) in vm_self.drain().with_id() {
            let mut player = (&mut vm_players).get(id)
                .expect("Player should exist");

            if msg.channel.asset == "brand" {
                tracing::debug!("Player brand is {}", msg.data.to_string());
                player.update_brand(msg.data.to_string());
            } else {
                tracing::debug!("Received plugin message on channel {}", msg.channel);
            }
        }
    }
}

#[packet(3)]
pub struct AcknowledgeFinishConfiguration;

#[state_changing]
impl PacketHandler for AcknowledgeFinishConfiguration {
    type Included<'a> = ViewMut<'a, Player>;

    fn handler<'a>(mut vm_self: ViewMut<Self>, mut vm_outgoing: ViewMut<'a, Bus<OutgoingPacket>>, mut vm_connection: ViewMut<'a, Connection>, mut vm_players: Self::Included<'a>) {
        for (id, _) in vm_self.drain().with_id() {
            let mut player = (&mut vm_connection).get(id)
                .expect("PlayerConnection should exist");

            if player.state != PlayerState::CONFIGURATION {
                tracing::error!("Received AcknowledgeFinishConfiguration from player {} despite state not being configuration.", player.username);
                continue;
            }

            player.state = PlayerState::PLAY;

            let mut player = (&mut vm_players).get(id)
                .expect("Player should exist");

            add_outgoing_packet(&mut vm_outgoing, id, PlayLogin {
                e_id: 0,
                is_hardcore: false,
                dimensions: vec![Identifier::new("minecraft", "overworld")],
                max_players: VarInt(100),
                view_distance: VarInt(player.actual_view_distance(100) as i32),
                simulation_distance: VarInt(player.actual_view_distance(100) as i32),
                reduced_debug_info: false,
                enable_respawns: true,
                limited_crafting: false,
                dimension_type: VarInt::from(0),
                dimension_name: Identifier::new("minecraft", "overworld"),
                seed: 0,
                game_mode: GameMode::Creative as u8,
                previous_game_mode: -1,
                is_debug: false,
                is_flat: true,
                death_location: None,
                portal_cooldown: VarInt(0),
                enforces_secure_chat: false,
            });

            add_outgoing_packet(&mut vm_outgoing, id, PlayerAbilities::default());
            
            for x in -3..3 {
                for z in -3..3 {
                    add_outgoing_packet(&mut vm_outgoing, id, ChunkDataAndUpdateLight {
                        x,
                        z,
                        heightmaps: empty_heightmap(),
                        data: vec![],
                        block_entities: vec![],
                        sky_light_mask: vec![],
                        block_light_mask: vec![],
                        empty_sky_light_mask: vec![],
                        empty_block_light_mask: vec![],
                        sky_light_array: vec![],
                        block_light_array: vec![],
                    })
                }
            }

            player.teleport(Location::new(0.0, 0.0, 0.0));
            add_outgoing_packet(&mut vm_outgoing, id, GameEvent {
                event: 13,
                value: 0.0
            })
        }
    }
}

#[packet(3)]
#[outgoing]
pub struct FinishConfiguration;

#[packet(0x07)]
#[outgoing]
pub struct RegistryData {
    registry_id: Identifier,
    entries: Vec<RegistryEntry>
}

#[derive(Deserializable, Serializable)]
struct RegistryEntry {
    id: Identifier,
    data: Option<NBT>
}

impl RegistryData {
    pub fn new(json: json::Result<JsonValue>) -> Self {
        let (registry_id, entries) = NBT::from_registry(json.expect("Failed to parse json"), true);

        Self {
            registry_id,
            entries: entries.into_iter().map(|(id, data)| RegistryEntry { id, data: Some(data) }).collect::<Vec<_>>()
        }
    }
}