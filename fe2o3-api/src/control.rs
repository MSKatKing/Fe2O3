use shipyard::{AllStoragesView, Unique};

#[derive(Unique)]
pub struct ShutdownRequest;

pub fn shutdown(storages: AllStoragesView) {
    storages.add_unique(ShutdownRequest);
}