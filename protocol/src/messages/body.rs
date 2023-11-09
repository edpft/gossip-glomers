use serde::{Deserialize, Serialize};

use super::{MessageId, Payload};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Body<P: Payload> {
    pub msg_id: Option<MessageId>,
    pub in_reply_to: Option<MessageId>,
    #[serde(flatten)]
    pub payload: P,
}

impl<P: Payload> Body<P> {
    pub fn new(
        msg_id: impl Into<Option<MessageId>>,
        in_reply_to: impl Into<Option<MessageId>>,
        payload: P,
    ) -> Self {
        Self {
            msg_id: msg_id.into(),
            in_reply_to: in_reply_to.into(),
            payload,
        }
    }

    pub fn msg_id(&self) -> Option<&MessageId> {
        self.msg_id.as_ref()
    }
}
