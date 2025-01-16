use libloading::{Library, Symbol};
use shipyard::{AllStoragesViewMut, IntoIter, Unique, UniqueView, Workload, World};

#[derive(Unique)]
struct PluginWorkload(Vec<(Library, Symbol<'static, fn() -> Workload>)>);

pub fn load_mods(world: &World) {
    // TODO: this is safe bc eventually we'll just look through a directory and won't assume dll/so files exist
    let mut workloads = Vec::new();

    unsafe {
        let lib = Library::new("target/debug/example_mod.dll").unwrap();

        let startup_workload: Symbol<fn() -> Workload> = lib.get("startup_workload".as_bytes()).unwrap();

        workloads = vec![(lib, startup_workload)];
    }

    world.add_unique(PluginWorkload(workloads));
}

pub fn plugin_update_workloads(storages: AllStoragesViewMut) {
    let plugin_workloads = storages.get_unique::<&PluginWorkload>().unwrap();
    for (_, workload) in &plugin_workloads.0 {
        storages.run(|| workload());
    }
}