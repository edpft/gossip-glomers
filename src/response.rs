use std::{collections::HashSet, fmt};

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Response {
    src: String,
    dest: String,
    body: ResponseBody,
}

impl Response {
    pub fn new(src: impl Into<String>, dest: impl Into<String>, body: ResponseBody) -> Self {
        Self {
            src: src.into(),
            dest: dest.into(),
            body,
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // fmt::Error does not support transmitting any information about an error other than that the error occurred.
        let string = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", string)
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct ResponseBody {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: ResponsePayload,
}

impl ResponseBody {
    pub fn new(
        msg_id: impl Into<Option<usize>>,
        in_reply_to: impl Into<Option<usize>>,
        payload: ResponsePayload,
    ) -> Self {
        Self {
            msg_id: msg_id.into(),
            in_reply_to: in_reply_to.into(),
            payload,
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ResponsePayload {
    // clippy doesn't like the repeated postfixes
    #[serde(rename = "init_ok")]
    Init,
    #[serde(rename = "echo_ok")]
    Echo { echo: String },
    #[serde(rename = "generate_ok")]
    Generate { id: Uuid },
    #[serde(rename = "broadcast_ok")]
    Broadcast,
    #[serde(rename = "read_ok")]
    Read { messages: Option<HashSet<usize>> },
    #[serde(rename = "topology_ok")]
    Topology,
}
