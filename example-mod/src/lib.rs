use std::any::TypeId;
use fe2o3_api::shipyard::{IntoWorkload, UniqueView, UniqueViewMut, Workload};
use fe2o3_api::ExperimentalStruct;
use fe2o3_api::tracing;

#[unsafe(no_mangle)]
pub fn startup_workload() -> Workload {
    (
        test,
        test_experimental,
    ).into_sequential_workload()
}

fn test() {
    tracing::info!("{:?}", TypeId::of::<ExperimentalStruct>());
}

fn test_experimental(mut test: UniqueViewMut<ExperimentalStruct>) {
    test.added_by = "Experimental".into();
}