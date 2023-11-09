use std::collections::HashSet;

use maelstrom_protocol::{messages::Payload, nodes::NodeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum CounterPayload {
    Init {
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
    },
    InitOk,
    Gossip {
        other_counts: Vec<u32>,
    },
    GossipOk {
        updated_counts: Vec<u32>,
    },
    Add {
        delta: u32,
    },
    AddOk,
    Read,
    ReadOk {
        value: u32,
    },
}

impl Payload for CounterPayload {}
