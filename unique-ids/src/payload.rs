use std::collections::HashSet;

use maelstrom_protocol::{messages::Payload, nodes::NodeId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum UniqueIdsPayload {
    Init {
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
    },
    InitOk,
    Generate,
    GenerateOk {
        id: Uuid,
    },
}

impl Payload for UniqueIdsPayload {}
