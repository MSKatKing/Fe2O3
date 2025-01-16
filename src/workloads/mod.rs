mod startup;
mod shutdown;
mod tick;

use shipyard::{IntoWorkload, Workload};
use crate::networking::packet::packet_handlers;
use crate::plugins::plugin_update_workloads;

pub fn startup() -> Workload {
    use startup::*;

    (
        setup_settings,
        setup_networking,
        setup_worlds,
    ).into_sequential_workload()
}

pub fn shutdown() -> Workload {
    use shutdown::*;

    (
        cleanup_networking,
    ).into_sequential_workload()
}

pub fn tick() -> Workload {
    use tick::*;

    (
        handle_networking_connection,
        handle_networking_incoming,

        packet_handlers, // Handle packets

        handle_teleport_requests,
        handle_keep_alives,

        handle_unsent_player_packets,

        handle_networking_outgoing,

        plugin_update_workloads,
    ).into_sequential_workload()
}