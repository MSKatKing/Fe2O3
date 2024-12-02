use shipyard::AllStoragesView;
use crate::game::world::WorldHandler;
use crate::networking::server::NetworkingHandler;
use crate::settings::{GameRules, ServerSettings};

pub fn setup_settings(storages: AllStoragesView) {
    storages.add_unique(ServerSettings::load());
}

pub fn setup_worlds(storages: AllStoragesView) {
    storages.add_unique(GameRules::new());
    storages.add_unique(WorldHandler::new());
}

pub fn setup_networking(storages: AllStoragesView) {
    let settings = storages.get_unique::<&ServerSettings>()
        .expect("ServerSettings did not exist");

    tracing::info!("Attempting to bind to server address...");
    storages.add_unique(NetworkingHandler::new(format!("{}:{}", settings.ip, settings.port)));

    storages.get_unique::<&NetworkingHandler>().expect("Should have networking manager").set_nonblocking(true)
        .expect("Failed to set handler to non-blocking mode");

    tracing::info!("Server started a {}:{}", settings.ip, settings.port);
}