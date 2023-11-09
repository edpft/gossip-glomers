use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::nodes::NodeId;

use super::{Body, MessageId, Payload};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Message<P: Payload> {
    pub src: NodeId,
    pub dest: NodeId,
    pub body: Body<P>,
}

impl<P: Payload + Serialize + Debug> Message<P> {
    pub fn new(
        src: impl Into<NodeId>,
        dest: impl Into<NodeId>,
        msg_id: impl Into<Option<MessageId>>,
        in_reply_to: impl Into<Option<MessageId>>,
        payload: P,
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

    pub fn src(&self) -> &NodeId {
        &self.src
    }

    pub fn dest(&self) -> &NodeId {
        &self.dest
    }

    pub fn msg_id(&self) -> Option<&MessageId> {
        self.body.msg_id()
    }
}

impl<P: Payload + Serialize> fmt::Display for Message<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // fmt::Error does not support transmitting any information about an error other than that the error occurred.
        let string = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", string)
    }
}
