use std::{collections::HashSet, fmt};

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::log::{LogKey, LogMessage, LogOffset, Messages, Offsets};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Message {
    pub src: NodeId,
    pub dest: NodeId,
    pub body: Body,
}

impl Message {
    pub fn new(
        src: impl Into<NodeId>,
        dest: impl Into<NodeId>,
        msg_id: impl Into<Option<usize>>,
        in_reply_to: impl Into<Option<usize>>,
        payload: Payload,
    ) -> Self {
        let body = Body::new(msg_id, in_reply_to, payload);
        Self {
            src: src.into(),
            dest: dest.into(),
            body,
        }
    }

    pub fn send(self) {
        println!("{}", &self);
        info!(target: "Sent message", message = ?self);
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

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Clone)]
pub struct NodeId(Box<str>);
