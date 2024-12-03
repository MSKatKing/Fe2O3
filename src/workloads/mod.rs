mod startup;
mod shutdown;
mod tick;

use shipyard::{IntoWorkload, Workload};
use crate::networking::packet::packet_handlers;

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

        handle_networking_outgoing,
    ).into_sequential_workload()
}