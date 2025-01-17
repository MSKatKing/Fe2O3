use libloading::Library;
use shipyard::{AllStoragesViewMut, Unique, Workload, WorkloadModificator, World};
use shipyard::error::RunWorkload;
use tracing::{error, info};

#[derive(Unique)]
struct PluginWorkload(Vec<Library>);

pub fn load_mods(world: &World) {
    // TODO: this is safe bc eventually we'll just look through a directory and won't assume dll/so files exist
    let mut libraries = Vec::new();

    unsafe {
        let lib = Library::new("target/debug/example_mod.dll").unwrap();
        let lib_name = lib.get::<fn() -> &'static str>("get_id".as_bytes()).unwrap()();

        if let Ok(workload) = lib.get::<fn() -> Workload>("startup_workload".as_bytes()) {
            workload.rename(format!("{lib_name}:startup"))
                .add_to_world(world)
                .unwrap();

            if let Err(err) = world.run_workload(format!("{}:startup", lib_name)) {
                error!("Failed to run workload '{}:startup': {}", lib_name, err);
            }
        }

        if let Ok(workload) = lib.get::<fn() -> Workload>("update_workload".as_bytes()) {
            workload.rename(format!("{lib_name}:update"))
                .add_to_world(world)
                .unwrap();
        }

        if let Ok(workload) = lib.get::<fn() -> Workload>("shutdown_workload".as_bytes()) {
            workload.rename(format!("{lib_name}:shutdown"))
                .add_to_world(world)
                .unwrap();
        }

        libraries.push(lib);
    }

    world.add_unique(PluginWorkload(libraries));
}

pub fn plugin_update_workloads(world: &World) {
    let plugin_workloads = world.get_unique::<&PluginWorkload>().unwrap();
    for library in &plugin_workloads.0 {
        unsafe {
            if let Ok(get_id) = library.get::<fn() -> &'static str>("get_id".as_bytes()) {
                let lib_name = get_id();
                if let Err(err) = world.run_workload(format!("{}:update", lib_name)) {
                    match err {
                        RunWorkload::MissingWorkload => { } // The plugin didn't register a workload of this type,
                        err => error!("Failed to run workload '{}:update': {}", lib_name, err),
                    }
                }
            } else {
                error!("Failed to load plugin id!");
            }
        }
    }
}

pub fn plugin_shutdown_workloads(world: &World) {
    let plugin_workloads = world.get_unique::<&PluginWorkload>().unwrap();
    for library in &plugin_workloads.0 {
        unsafe {
            if let Ok(get_id) = library.get::<fn() -> &'static str>("get_id".as_bytes()) {
                let lib_name = get_id();
                if let Err(err) = world.run_workload(format!("{}:shutdown", lib_name)) {
                    match err {
                        RunWorkload::MissingWorkload => { } // The plugin didn't register a workload of this type,
                        err => error!("Failed to run workload '{}:shutdown': {}", lib_name, err),
                    }
                }
            } else {
                error!("Failed to load plugin id!");
            }
        }
    }
}