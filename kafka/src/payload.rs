use std::collections::HashSet;

use maelstrom_protocol::{messages::Payload, nodes::NodeId};
use serde::{Deserialize, Serialize};

use crate::log::{LogKey, LogMessage, LogOffset, Messages, Offsets};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum KafkaPayload {
    Init {
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
    },
    InitOk,
    Send {
        key: LogKey,
        msg: LogMessage,
    },
    SendOk {
        offset: LogOffset,
    },
    Poll {
        offsets: Offsets,
    },
    PollOk {
        msgs: Messages,
    },
    CommitOffsets {
        offsets: Offsets,
    },
    CommitOffsetsOk,
    ListCommittedOffsets {
        keys: Vec<LogKey>,
    },
    ListCommittedOffsetsOk {
        offsets: Offsets,
    },
}

impl Payload for KafkaPayload {}
