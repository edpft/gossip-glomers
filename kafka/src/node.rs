use std::collections::HashSet;

use maelstrom_protocol::{
    messages::{Message, MessageId},
    nodes::{Node, NodeId},
};
use tracing::{error, info};

use crate::{log::Logs, payload::KafkaPayload};

#[derive(Debug)]
pub enum KafkaNode {
    Uninitialised {
        msg_id: MessageId,
    },
    Initialised {
        msg_id: MessageId,
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
        logs: Logs,
    },
}

impl Node<KafkaPayload> for KafkaNode {
    fn new() -> Self {
        let msg_id = MessageId::default();
        Self::Uninitialised { msg_id }
    }

    fn id(&self) -> Option<&NodeId> {
        match self {
            Self::Uninitialised { .. } => None,
            Self::Initialised {
                msg_id: _,
                node_id,
                node_ids: _,
                logs: _,
            } => Some(node_id),
        }
    }

    fn msg_id(&self) -> &MessageId {
        match self {
            Self::Uninitialised { msg_id }
            | Self::Initialised {
                msg_id,
                node_id: _,
                node_ids: _,
                logs: _,
            } => msg_id,
        }
    }

    fn handle(self, request: Message<KafkaPayload>) -> Self {
        info!(target: "Received message", message = ?request);
        let (node, payload) = match self {
            Self::Uninitialised { msg_id } => match &request.body.payload {
                KafkaPayload::Init { node_id, node_ids } => {
                    let logs = Logs::default();
                    let node = Self::Initialised {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        logs,
                    };
                    let payload = KafkaPayload::InitOk;
                    (node, Some(payload))
                }
                payload => {
                    error!(target: "invalid payload", node_type = "Uninitialised", payload = ?payload);
                    let node = Self::Uninitialised { msg_id };
                    (node, None)
                }
            },
            Self::Initialised {
                msg_id,
                node_id,
                node_ids,
                mut logs,
            } => match &request.body.payload {
                KafkaPayload::Send { key, msg } => {
                    let offset = logs.append_message(key.clone(), msg.clone());
                    let node = Self::Initialised {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids,
                        logs,
                    };
                    let payload = KafkaPayload::SendOk { offset };
                    (node, Some(payload))
                }
                KafkaPayload::Poll { offsets } => {
                    let messages = logs.since_offset(offsets.clone()).as_messages();
                    let node = Self::Initialised {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids,
                        logs,
                    };
                    let payload = KafkaPayload::PollOk { msgs: messages };
                    (node, Some(payload))
                }
                KafkaPayload::CommitOffsets { offsets } => {
                    logs.commit_offsets(offsets.clone());
                    let node = Self::Initialised {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids,
                        logs,
                    };
                    let payload = KafkaPayload::CommitOffsetsOk;
                    (node, Some(payload))
                }
                KafkaPayload::ListCommittedOffsets { keys } => {
                    let offsets = logs.list_committed_offsets(keys.clone());
                    let node = Self::Initialised {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids,
                        logs,
                    };
                    let payload = KafkaPayload::ListCommittedOffsetsOk { offsets };
                    (node, Some(payload))
                }
                payload => {
                    error!(target: "invalid payload", node_type = "Initialised", payload = ?payload);
                    let node = Self::Initialised {
                        msg_id,
                        node_id,
                        node_ids,
                        logs,
                    };
                    (node, None)
                }
            },
        };
        if let Some(payload) = payload {
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
