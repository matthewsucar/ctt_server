use crate::entities::target::TargetStatus;
use std::collections::HashMap;

pub trait SchedulerTrait: std::fmt::Debug + Send + Sync{
    fn nodes_status(&mut self) -> Result<HashMap<String, (TargetStatus, String)>, String>;
    fn release_node(&mut self, target: &str) -> Result<(), ()>;
    fn offline_node(&mut self, target: &str, comment: &str) -> Result<(), ()>;
}
#[cfg(feature="pbs")]
mod pbs_scheduler;
mod null_scheduler;

#[cfg(feature="pbs")]
pub use pbs_scheduler::PbsScheduler;

pub use null_scheduler::NullScheduler;
