use crate::messages::{Message, MessageId, Payload};

use super::NodeId;

pub trait Node<P: Payload> {
    fn new() -> Self;

    fn id(&self) -> Option<&NodeId>;

    fn msg_id(&self) -> &MessageId;

    fn handle(self, request: Message<P>) -> Self;
}
