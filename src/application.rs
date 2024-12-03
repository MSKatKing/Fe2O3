use std::thread;
use std::time::{Duration, Instant};
use shipyard::World;
use crate::status_tags::Shutdown;
use crate::workloads;

pub struct Application {
    ecs_world: World
}

impl Application {
    pub fn new() -> Self {
        let ecs_world = World::new();

        ecs_world.add_workload(workloads::startup);
        ecs_world.add_workload(workloads::shutdown);
        ecs_world.add_workload(workloads::tick);

        tracing::info!("Workloads initialized.");

        ecs_world.set_default_workload(workloads::tick)
            .expect("Failed to set default workload to 'tick'");

        tracing::info!("Running startup workload...");

        ecs_world.run_workload(workloads::startup)
            .expect("Failed to execute startup workload");

        tracing::info!("Server startup successful!");

        Self {
            ecs_world
        }
    }

    pub fn run(self) {
        const TARGET_TICK_DURATION: Duration = Duration::from_millis(1000 / 20);

        loop {
            if self.ecs_world.get_unique::<&Shutdown>().is_ok() {
                break;
            }

            let start_time = Instant::now();

            self.ecs_world.run_default_workload()
                .expect("Failed to run tick workload");

            let tick_duration = start_time.elapsed();

            if let Some(remaining_time) = TARGET_TICK_DURATION.checked_sub(tick_duration) {
                thread::sleep(remaining_time);
            } else {
                let over_time = tick_duration - TARGET_TICK_DURATION;
                let over_time_ticks = (over_time.as_millis() / TARGET_TICK_DURATION.as_millis()).max(1);

                // TODO: change this to an option in the config
                if over_time_ticks > 10 {
                    tracing::warn!("Server overloaded! {} tick(s) ({}ms) behind!", over_time_ticks, over_time.as_millis());
                }
            }
        }

        self.ecs_world.run_workload(workloads::shutdown)
            .expect("Failed to run workload shutdown");
    }
}