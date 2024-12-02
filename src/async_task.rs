use std::time::Duration;
use crossbeam_channel::{SendError, Sender};
use shipyard::Unique;

#[derive(Unique)]
pub struct AsyncTaskSubmitter(Sender<AsyncTask>);

impl AsyncTaskSubmitter {
    pub fn send(&self, task: impl FnOnce() + Send + Sync + 'static) -> Result<(), SendError<AsyncTask>> {
        self.0.send(Box::new(task))
    }
}

#[derive(Unique)]
pub struct DelayedTaskSubmitter(Sender<DelayedTask>);

impl DelayedTaskSubmitter {
    pub fn send(&self, task: impl FnOnce() + Send + Sync + 'static, wait_for: Duration) -> Result<(), SendError<DelayedTask>> {
        self.0.send((Box::new(task), wait_for))
    }
}

pub type AsyncTask = Box<dyn FnOnce() + Send + Sync>;
pub type DelayedTask = (Box<dyn FnOnce() + Send + Sync>, Duration);