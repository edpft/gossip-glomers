use std::collections::{HashMap, HashSet};

use maelstrom_protocol::{messages::Payload, nodes::NodeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum BroadcastPayload {
    Init {
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
    },
    InitOk,
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: HashSet<usize>,
    },
    Topology {
        topology: HashMap<NodeId, HashSet<NodeId>>,
    },
    TopologyOk,
    Gossip {
        ids_to_see: HashSet<usize>,
    },
    GossipOk {
        ids_to_see: HashSet<usize>,
    },
}

impl Payload for BroadcastPayload {}
