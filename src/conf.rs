use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use nodeset::NodeSet;

pub fn get_config(path: Option<String>) -> Result<Conf, ConfigError> {
    let mut conf = Config::builder();
    if let Some(p) = path {
        conf = conf.add_source(File::with_name(&p));
    }
    let conf = conf.build()?;
    let mut conf: Conf = match conf.try_deserialize() {
        Ok(conf) => conf,
        Err(e) => return Err(e),
    };

    for nr in &conf.nodelist {
        let ns: NodeSet = nr.parse().unwrap(); //TODO error prop
        for n in ns.iter() {
            conf.expanded_nodes.push(n);
        }
    }
    return Ok(conf);
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Conf {
    pub poll_interval: u64,
    pub slack: Slack,
    pub db: String,
    pub certs_dir: String,
    pub server_addr: String,
    pub node_types: Vec<NodeType>,
    pub auth: Auth,
#[serde(default)]
    pub nodelist: Vec<String>,
#[serde(skip)]
    pub expanded_nodes: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Auth {
    pub admin: Vec<String>,
    pub guest: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Slack {
    pub channel: String,
    pub token: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NodeType {
    pub prefix: String,
    pub digits: Option<usize>,
    pub board: Option<u32>,
    pub first_num: Option<u32>,
    pub last_num: Option<u32>,
    pub slot: Option<u32>,
}
