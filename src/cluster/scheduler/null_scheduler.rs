use crate::entities::target::TargetStatus;
use core::fmt;
use std::collections::HashMap;
use tracing::instrument;
use tracing::{info, warn};

use super::SchedulerTrait;
#[derive(Clone,Debug)]
struct FakeNode {
    hostname: String,
    state: TargetStatus,
    comment: String,
}
pub struct NullScheduler {
    nodes : HashMap<String, FakeNode>,
}

impl NullScheduler {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn add_node(&mut self, name: &str) -> Result<(), ()> {
        let n = FakeNode { hostname : name.to_string(), state : TargetStatus::Online, comment : String::new(),};
        self.nodes.insert(name.to_string(), n);
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
        let mut resp = HashMap::new();
        for (hn, n) in &self.nodes {
            resp.insert(hn.to_string(), (n.state, n.comment.to_string()));
        }
        Ok(resp)
    }

    #[instrument]
    fn release_node(&mut self, target: &str) -> Result<(), ()> {
        info!("resuming node {}", target);
        let node = match self.nodes.get_mut(&target.to_string()) {
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
        let node = match self.nodes.get_mut(&target.to_string()) {
            Some(n) => n,
            None => return Err(()),
        };
        node.state = TargetStatus::Offline;
        node.comment = comment.to_string();
        Ok(())
    }
}