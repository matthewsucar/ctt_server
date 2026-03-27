use crate::entities::target::TargetStatus;
use core::fmt;
use std::collections::HashMap;
use tracing::instrument;
use tracing::{info, warn};
use std::sync::{Arc, LazyLock, Mutex};

use super::SchedulerTrait;

#[derive(Clone,Debug)]
struct FakeNode {
    #[allow(dead_code)]
    hostname: String,
    state: TargetStatus,
    comment: String,
}

static NODESTATE: LazyLock<Arc<Mutex<HashMap<String, FakeNode>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});
pub struct NullScheduler {
}

impl NullScheduler {
    pub fn new() -> Self {
        let mut states = NODESTATE.lock().unwrap();
        if states.is_empty() {
            for cfgnode in &crate::CONFIG.get().unwrap().expanded_nodes {
                let faken = FakeNode { hostname : cfgnode.to_string(), state : TargetStatus::Online, comment : String::new(),};
                states.insert(cfgnode.to_string(), faken);
            }
        }
        Self { }
    }

    #[allow(dead_code)]
    pub fn add_node(&mut self, name: &str) -> Result<(), ()> {
        let mut states = NODESTATE.lock().unwrap();
        let n = FakeNode { hostname : name.to_string(), state : TargetStatus::Online, comment : String::new(),};
        states.insert(name.to_string(), n);
        Ok(())
    }

}

impl fmt::Debug for NullScheduler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NullScheduler").finish()
    }
}

impl Default for NullScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedulerTrait for NullScheduler {
    #[instrument]
    fn nodes_status(&mut self) -> Result<HashMap<String, (TargetStatus, String)>, String> {
        let states = NODESTATE.lock().unwrap();
        let mut resp = HashMap::new();
        for (hn, n) in states.iter() {
            resp.insert(hn.to_string(), (n.state, n.comment.to_string()));
        }
        Ok(resp)
    }

    #[instrument]
    fn release_node(&mut self, target: &str) -> Result<(), ()> {
        info!("resuming node {}", target);
        let mut states = NODESTATE.lock().unwrap();
        let node = match states.get_mut(&target.to_string()) {
            Some(n) => n,
            None => return Err(()),
        };
        node.state = TargetStatus::Online;
        node.comment = "".to_string();
        Ok(())
    }

    #[instrument]
    fn offline_node(&mut self, target: &str, comment: &str) -> Result<(), ()> {
        info!("offlining: {}, {}", target, comment);
        let mut states = NODESTATE.lock().unwrap();
        let node = match states.get_mut(&target.to_string()) {
            Some(n) => n,
            None => return Err(()),
        };
        node.state = TargetStatus::Offline;
        node.comment = comment.to_string();
        Ok(())
    }
}