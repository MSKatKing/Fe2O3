use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use shipyard::Unique;
use packet::Identifier;

#[derive(Unique, serde::Serialize, serde::Deserialize)]
pub struct ServerSettings {
    pub ip: String,
    pub port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: 25565
        }
    }
}

impl ServerSettings {
    pub fn load() -> Self {
        let mut file = OpenOptions::new().create(true).read(true).write(true).open("config.toml").expect("Failed to open config file");
        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string).expect("Failed to read config file");

        toml::from_str(&toml_string).unwrap_or_else(|_| {
            tracing::warn!("Failed to parse config, using default values.");
            drop(file);
            ServerSettings::default().save();
            ServerSettings::default()
        })
    }

    pub fn save(&self) {
        let toml_string = toml::to_string(self).expect("Failed to save config to string");
        let mut file = OpenOptions::new().create(true).write(true).open("config.toml").expect("Failed to open config file");
        file.write_all(toml_string.as_bytes()).expect("Failed to write to config file");
    }
}

#[derive(Unique)]
pub struct GameRules {
    map: HashMap<Identifier, GameRule>
}

impl GameRules {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn get_gamerules_or_load(&mut self, world: Identifier) -> &GameRule {
        self.map.entry(world).or_default()
    }

    pub fn get_gamerules_or_load_mut(&mut self, world: Identifier) -> &mut GameRule {
        self.map.entry(world).or_default()
    }
}

pub struct GameRule {
    pub announce_advancements: bool,
    pub block_explosion_drop_decay: bool,
    pub command_block_output: bool,
    pub command_modification_block_limit: u32,
    pub disable_elytra_movement_check: bool,
    pub disable_raids: bool,
    pub do_daylight_cycle: bool,
    pub do_entity_drops: bool,
    pub do_fire_tick: bool,
    pub do_insomnia: bool,
    pub do_immediate_respawn: bool,
    pub do_limited_crafting: bool,
    pub do_mob_loot: bool,
    pub do_mob_spawning: bool,
    pub do_patrol_spawning: bool,
    pub do_tile_drops: bool,
    pub do_trader_spawning: bool,
    pub do_vines_spread: bool,
    pub do_weather_cycle: bool,
    pub do_warden_spawning: bool,
    pub drowning_damage: bool,
    pub ender_pearls_vanish_on_death: bool,
    pub fall_damage: bool,
    pub fire_damage: bool,
    pub forgive_dead_players: bool,
    pub freeze_damage: bool,
    pub global_sound_events: bool,
    pub keep_inventory: bool,
    pub lava_source_conversion: bool,
    pub log_admin_commands: bool,
    pub max_command_chain_length: u32,
    pub max_command_fork_count: u32,
    pub max_entity_cramming: u32,
    pub mob_explosion_drop_decay: bool,
    pub mob_griefing: bool,
    pub natural_regeneration: bool,
    pub players_nether_portal_creative_delay: u32,
    pub players_nether_portal_default_delay: u32,
    pub players_sleeping_percentage: u32,
    pub projectiles_can_break_blocks: bool,
    pub random_tick_speed: u32,
    pub reduced_debug_info: bool,
    pub send_command_feedback: bool,
    pub show_death_messages: bool,
    pub snow_accumulation_height: u32,
    pub spawn_chunk_radius: u32,
    pub spawn_radius: u32,
    pub spectators_generate_chunks: bool,
    pub tnt_explosion_drop_decay: bool,
    pub universal_anger: bool,
    pub water_source_conversion: bool
}

impl Default for GameRule {
    fn default() -> Self {
        Self {
            announce_advancements: true,
            block_explosion_drop_decay: true,
            command_block_output: true,
            command_modification_block_limit: 32768,
            disable_elytra_movement_check: false,
            disable_raids: false,
            do_daylight_cycle: true,
            do_entity_drops: true,
            do_fire_tick: true,
            do_insomnia: true,
            do_immediate_respawn: false,
            do_limited_crafting: false,
            do_mob_loot: true,
            do_mob_spawning: true,
            do_patrol_spawning: true,
            do_tile_drops: true,
            do_trader_spawning: true,
            do_vines_spread: true,
            do_weather_cycle: true,
            do_warden_spawning: true,
            drowning_damage: true,
            ender_pearls_vanish_on_death: true,
            fall_damage: true,
            fire_damage: true,
            forgive_dead_players: true,
            freeze_damage: true,
            global_sound_events: true,
            keep_inventory: false,
            lava_source_conversion: false,
            log_admin_commands: true,
            max_command_chain_length: 65536,
            max_command_fork_count: 65536,
            max_entity_cramming: 24,
            mob_explosion_drop_decay: true,
            mob_griefing: true,
            natural_regeneration: true,
            players_nether_portal_creative_delay: 1,
            players_nether_portal_default_delay: 80,
            players_sleeping_percentage: 100,
            projectiles_can_break_blocks: true,
            random_tick_speed: 3,
            reduced_debug_info: false,
            send_command_feedback: true,
            show_death_messages: true,
            snow_accumulation_height: 1,
            spawn_chunk_radius: 2,
            spawn_radius: 10,
            spectators_generate_chunks: true,
            tnt_explosion_drop_decay: false,
            universal_anger: false,
            water_source_conversion: true,
        }
    }
}