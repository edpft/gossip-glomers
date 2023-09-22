use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn new(src: impl Into<String>, dest: impl Into<String>, body: Body) -> Self {
        Self {
            src: src.into(),
            dest: dest.into(),
            body,
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // fmt::Error does not support transmitting any information about an error other than that the error occurred.
        let string = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", string)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Body {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

impl Body {
    pub fn new(
        msg_id: impl Into<Option<usize>>,
        in_reply_to: impl Into<Option<usize>>,
        payload: Payload,
    ) -> Self {
        Self {
            msg_id: msg_id.into(),
            in_reply_to: in_reply_to.into(),
            payload,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Payload {
    Init {
        node_id: String,
        node_ids: HashSet<String>,
    },
    InitOk,
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Generate,
    GenerateOk {
        id: Uuid,
    },
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Option<HashSet<usize>>,
    },
    Topology {
        topology: HashMap<String, HashSet<String>>,
    },
    TopologyOk,
    Gossip {
        ids_to_see: Option<HashSet<usize>>,
    },
    GossipOk {
        ids_seen: Option<HashSet<usize>>,
    },
}
