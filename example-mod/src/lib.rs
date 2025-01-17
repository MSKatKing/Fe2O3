use fe2o3_api::shipyard::{AllStoragesView, IntoWorkload, Workload};
use fe2o3_api::control::shutdown;

#[no_mangle]
pub fn get_id() -> &'static str {
    "experimental_mod"
}

#[no_mangle]
pub fn startup_workload() -> Workload {
    (
        MyMod::on_load,
    ).into_sequential_workload()
}

#[no_mangle]
pub fn shutdown_workload() -> Workload {
    (
        MyMod::on_shutdown,
    ).into_sequential_workload()
}

pub struct MyMod;

impl MyMod {
    fn on_load(storages: AllStoragesView) {
        shutdown(storages);
        println!("shutdown requested!");
    }

    fn on_shutdown() {
        println!("Goodbye!");
    }
}