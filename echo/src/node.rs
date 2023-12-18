use std::collections::HashSet;

use maelstrom_protocol::{
    messages::{Message, MessageId},
    nodes::{Node, NodeId},
};
use tracing::{error, info};

use crate::payload::EchoPayload;

#[derive(Debug, PartialEq, Eq)]
pub enum EchoNode {
    Uninitialised {
        msg_id: MessageId,
    },
    Initialised {
        msg_id: MessageId,
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
    },
}

impl Node<EchoPayload> for EchoNode {
    fn new() -> Self {
        let msg_id = MessageId::default();
        EchoNode::Uninitialised { msg_id }
    }

    fn id(&self) -> Option<&NodeId> {
        match self {
            EchoNode::Uninitialised { .. } => None,
            EchoNode::Initialised {
                msg_id: _,
                node_id,
                node_ids: _,
            } => Some(node_id),
        }
    }

    fn msg_id(&self) -> &MessageId {
        match self {
            EchoNode::Uninitialised { msg_id } => msg_id,
            EchoNode::Initialised {
                msg_id,
                node_id: _,
                node_ids: _,
            } => msg_id,
        }
    }

    fn handle(self, request: Message<EchoPayload>) -> Self {
        info!(target: "Received message", message = ?request);
        let (node, response_payload) = match self {
            EchoNode::Uninitialised { msg_id } => match &request.body.payload {
                EchoPayload::Init { node_id, node_ids } => {
                    let node = EchoNode::Initialised {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                    };
                    let response_payload = EchoPayload::InitOk;
                    (node, Some(response_payload))
                }
                unexpected_payload => {
                    error!(target: "invalid payload", node_type = "Uninitialised", payload = ?unexpected_payload);
                    let node = EchoNode::Uninitialised { msg_id };
                    (node, None)
                }
            },
            EchoNode::Initialised {
                msg_id,
                node_id,
                node_ids,
            } => match &request.body.payload {
                EchoPayload::Echo { echo } => {
                    let node = EchoNode::Initialised {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                    };
                    let response_payload = EchoPayload::EchoOk {
                        echo: echo.to_owned(),
                    };
                    (node, Some(response_payload))
                }
                unexpected_payload => {
                    error!(target: "invalid payload", node_type = "Uninitialised", payload = ?unexpected_payload);
                    let node = EchoNode::Uninitialised { msg_id };
                    (node, None)
                }
            },
        };
        if let Some(payload) = response_payload {
            let response = Message::new(
                request.dest().clone(),
                request.src().clone(),
                node.msg_id().clone(),
                request.msg_id().cloned(),
                payload,
            );
            response.send();
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uninitialised_node_handles_init_message() {
        let initial_node = EchoNode::new();
        let sender_id = NodeId::new(0);
        let receiver_id = NodeId::new(1);
        let message_id = MessageId::new(0);
        let node_ids = HashSet::from([receiver_id.clone()]);
        let payload = EchoPayload::Init {
            node_id: receiver_id.clone(),
            node_ids: node_ids.clone(),
        };
        let message = Message::new(
            sender_id,
            receiver_id.clone(),
            message_id.clone(),
            None,
            payload,
        );
        let subsequent_node = initial_node.handle(message);
        assert_eq!(
            subsequent_node,
            EchoNode::Initialised {
                msg_id: message_id.increment(),
                node_id: receiver_id,
                node_ids
            }
        )
    }
}
