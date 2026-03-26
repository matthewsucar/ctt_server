
use crate::cluster::scheduler::{SchedulerTrait};
#[cfg(feature = "pbs")]
use crate::cluster::scheduler::PbsScheduler;
use crate::cluster::scheduler::NullScheduler;
use crate::Conf;

pub fn get_scheduler(conf: &Conf) -> Box<dyn SchedulerTrait> {
    let mut have_pbs:bool = false;
    if cfg!(feature = "pbs") {
        have_pbs = true;
    }
    if conf.backend.is_empty() && have_pbs {
        #[cfg(feature = "pbs")]
        return Box::new(PbsScheduler::new());
    }
    if !conf.backend.is_empty() {
        match conf.backend.to_string().to_ascii_lowercase().trim() {
            #[cfg(feature = "pbs")]
            "pbs" => return Box::new(PbsScheduler::new()),
            "null" => return Box::new(NullScheduler::new()),
            _ => return Box::new(NullScheduler::new()),
        }
    }
    return Box::new(NullScheduler::new())
}