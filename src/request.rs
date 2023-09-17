use std::collections::{HashMap, HashSet};

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Request {
    pub src: String,
    pub dest: String,
    pub body: RequestBody,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RequestBody {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: RequestPayload,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum RequestPayload {
    Init {
        node_id: String,
        node_ids: HashSet<String>,
    },
    Echo {
        echo: String,
    },
    Generate,
    Broadcast {
        message: usize,
    },
    Read,
    Topology {
        topology: HashMap<String, HashSet<String>>,
    },
}
