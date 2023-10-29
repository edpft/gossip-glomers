use std::{collections::HashSet, thread, time::Duration};

use tracing::{error, info};

use crate::{
    counter::GrowOnlyCounter,
    message::{Message, NodeId, Payload},
};

#[derive(Debug)]
pub enum Node {
    Uninitialised {
        msg_id: usize,
    },
    Initialised {
        msg_id: usize,
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
        counter: GrowOnlyCounter,
    },
}

impl Node {
    pub fn new() -> Self {
        Node::Uninitialised { msg_id: 0 }
    }
}

impl Node {
    pub fn handle(self, request: Message) -> Self {
        info!(target: "Received message", message = ?request);
        match self {
            Node::Uninitialised { msg_id } => match request.body.payload {
                Payload::Init { node_id, node_ids } => {
                    handle_init_request(msg_id, node_id, node_ids, request.src, request.body.msg_id)
                }
                payload => {
                    error!(target: "invalid payload", node_type = "Uninitialised", payload = ?payload);
                    Node::Uninitialised { msg_id }
                }
            },
            Node::Initialised {
                msg_id,
                node_id,
                node_ids,
                mut counter,
            } => match request.body.payload {
                Payload::Add { delta } => {
                    let Some(()) = counter.add_to_count(node_id.id_number(), delta) else {
                        todo!();
                    };
                    let node = Node::Initialised {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        counter,
                    };
                    let response_payload = Payload::AddOk;
                    let response = Message::new(
                        node_id,
                        request.src,
                        msg_id,
                        request.body.msg_id,
                        response_payload,
                    );
                    response.send();
                    node
                }
                Payload::Read => {
                    let value = counter.sum();
                    let node = Node::Initialised {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        counter,
                    };
                    let response_payload = Payload::ReadOk { value };
                    let response = Message::new(
                        node_id,
                        request.src,
                        msg_id,
                        request.body.msg_id,
                        response_payload,
                    );
                    response.send();
                    node
                }
                Payload::Gossip { other_counts } => {
                    counter.update_counts(&other_counts);
                    let updated_counts = counter.counts().to_owned();
                    let node = Node::Initialised {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        counter,
                    };
                    let response_payload = Payload::GossipOk { updated_counts };
                    let response = Message::new(
                        node_id,
                        request.src,
                        msg_id,
                        request.body.msg_id,
                        response_payload,
                    );
                    response.send();
                    node
                }
                Payload::GossipOk { updated_counts } => {
                    counter.update_counts(&updated_counts);
                    Node::Initialised {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        counter,
                    }
                }
                payload => {
                    error!(target: "invalid payload", node_type = "Initialised", payload = ?payload);
                    Node::Initialised {
                        msg_id,
                        node_id,
                        node_ids,
                        counter,
                    }
                }
            },
        }
    }
    pub fn gossip(&self) {
        let Node::Initialised {
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
                let payload = Payload::Gossip {
                    other_counts: counts,
                };
                let request =
                    Message::new(node_id.clone(), neighbour_id.clone(), None, None, payload);
                request.send();
            })
    }
}

fn handle_init_request(
    msg_id: usize,
    node_id: NodeId,
    node_ids: HashSet<NodeId>,
    dest: NodeId,
    in_reply_to: impl Into<Option<usize>>,
) -> Node {
    let node_count = node_ids.len();
    let counter = GrowOnlyCounter::new(node_count);
    let node = Node::Initialised {
        msg_id: msg_id + 1,
        node_id: node_id.clone(),
        node_ids,
        counter,
    };
    let response_payload = Payload::InitOk;
    let response = Message::new(node_id, dest, msg_id, in_reply_to, response_payload);
    response.send();
    node
}
