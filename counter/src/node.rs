use std::{collections::HashSet, thread, time::Duration};

use maelstrom_protocol::{
    gossip::Gossip,
    messages::{Message, MessageId},
    nodes::{Node, NodeId},
};
use tracing::{error, info};

use crate::{counter::GrowOnlyCounter, payload::CounterPayload};

#[derive(Debug)]
pub enum CounterNode {
    Uninitialised {
        msg_id: MessageId,
    },
    Initialised {
        msg_id: MessageId,
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
        counter: GrowOnlyCounter,
    },
}

impl Node<CounterPayload> for CounterNode {
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
                counter: _,
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
                counter: _,
            } => msg_id,
        }
    }

    fn handle(self, request: Message<CounterPayload>) -> Self {
        info!(target: "Received message", message = ?request);
        let (node, response_payload) = match self {
            Self::Uninitialised { msg_id } => match &request.body.payload {
                CounterPayload::Init { node_id, node_ids } => {
                    let node_count = node_ids.len();
                    let counter = GrowOnlyCounter::new(node_count);
                    let node = Self::Initialised {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        counter,
                    };
                    let payload = CounterPayload::InitOk;
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
                mut counter,
            } => match &request.body.payload {
                CounterPayload::Add { delta } => {
                    let Some(()) = counter.add_to_count(node_id.id_number(), *delta) else {
                        todo!();
                    };
                    let node = Self::Initialised {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        counter,
                    };
                    let payload = CounterPayload::AddOk;
                    (node, Some(payload))
                }
                CounterPayload::Read => {
                    let value = counter.sum();
                    let node = Self::Initialised {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        counter,
                    };
                    let payload = CounterPayload::ReadOk { value };
                    (node, Some(payload))
                }
                CounterPayload::Gossip { other_counts } => {
                    counter.update_counts(other_counts);
                    let updated_counts = counter.counts().to_owned();
                    let node = Self::Initialised {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        counter,
                    };
                    let payload = CounterPayload::GossipOk { updated_counts };
                    (node, Some(payload))
                }
                CounterPayload::GossipOk { updated_counts } => {
                    counter.update_counts(updated_counts);
                    let node = Self::Initialised {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        counter,
                    };
                    (node, None)
                }
                payload => {
                    error!(target: "invalid payload", node_type = "Initialised", payload = ?payload);
                    let node = Self::Initialised {
                        msg_id,
                        node_id,
                        node_ids,
                        counter,
                    };
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

impl Gossip for CounterNode {
    fn gossip(&self) {
        let CounterNode::Initialised {
            msg_id: _,
            node_id,
            node_ids,
            counter,
        } = self
        else {
            return;
        };
        let id_number = node_id.id_number();
        let duration = Duration::from_micros(id_number as u64);
        thread::sleep(duration);
        node_ids
            .iter()
            .filter(|neighbour_id| neighbour_id != &node_id)
            .for_each(|neighbour_id| {
                let counts = counter.counts().to_owned();
                let payload = CounterPayload::Gossip {
                    other_counts: counts,
                };
                let request =
                    Message::new(node_id.clone(), neighbour_id.clone(), None, None, payload);
                request.send();
            })
    }
}
