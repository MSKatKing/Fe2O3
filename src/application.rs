use std::thread;
use std::time::{Duration, Instant};
use crossbeam_channel::Receiver;
use shipyard::World;
use crate::async_task::AsyncTask;
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
                tracing::warn!("Warning: Server overloaded! {}ms behind!", over_time.as_millis());
            }
        }

        self.ecs_world.run_workload(workloads::shutdown)
            .expect("Failed to run workload shutdown");
    }

    fn async_run(rx: Receiver<AsyncTask>) {
        let mut handlers = Vec::new();

        while let Ok(task) = rx.recv() {
            handlers.push(thread::spawn(task));
        }

        for handler in handlers {
            handler.join().unwrap_or_default()
        }
    }
}