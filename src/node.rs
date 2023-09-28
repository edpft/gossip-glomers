use std::{
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

use tracing::{error, info};
use uuid::Uuid;

use crate::message::{Message, NodeId, Payload};

#[derive(Debug)]
pub enum Node {
    Uninitialised {
        msg_id: usize,
    },
    Initialised {
        msg_id: usize,
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
    },
    Networked {
        msg_id: usize,
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
        ids_seen_by_neighbours: IdsSeenByNeighbours,
    },
    Broadcasting {
        msg_id: usize,
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
        ids_seen: HashSet<usize>,
    },
    NetworkedBroadcasting {
        msg_id: usize,
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
        ids_seen: HashSet<usize>,
        ids_seen_by_neighbours: IdsSeenByNeighbours,
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
            } => match request.body.payload {
                Payload::Echo { echo } => handle_echo_request(
                    msg_id,
                    node_id,
                    node_ids,
                    echo,
                    request.src,
                    request.body.msg_id,
                ),
                Payload::Generate => handle_generate_request(
                    msg_id,
                    node_id,
                    node_ids,
                    request.src,
                    request.body.msg_id,
                ),
                Payload::Topology { topology } => handle_topology_request(
                    msg_id,
                    node_id,
                    node_ids,
                    topology,
                    request.src,
                    request.body.msg_id,
                ),
                Payload::Broadcast { message } => {
                    let mut ids_seen = HashSet::new();
                    ids_seen.insert(message);
                    let node = Node::Broadcasting {
                        msg_id: msg_id + 1,
                        node_id: node_id.clone(),
                        node_ids,
                        ids_seen,
                    };
                    let payload = Payload::BroadcastOk;
                    let response =
                        Message::new(node_id, request.src, msg_id, request.body.msg_id, payload);
                    response.send();
                    node
                }
                payload => {
                    error!(target: "invalid payload", node_type = "Initialised", payload = ?payload);
                    Node::Initialised {
                        msg_id,
                        node_id,
                        node_ids,
                    }
                }
            },
            Node::Broadcasting {
                msg_id,
                node_id,
                node_ids,
                mut ids_seen,
            } => match request.body.payload {
                Payload::Broadcast { message } => {
                    ids_seen.insert(message);
                    let node = Node::Broadcasting {
                        msg_id: msg_id + 1,
                        node_id: node_id.clone(),
                        node_ids,
                        ids_seen,
                    };
                    let payload = Payload::BroadcastOk;
                    let response =
                        Message::new(node_id, request.src, msg_id, request.body.msg_id, payload);
                    response.send();
                    node
                }
                Payload::Read => {
                    let node = Node::Broadcasting {
                        msg_id: msg_id + 1,
                        node_id: node_id.clone(),
                        node_ids,
                        ids_seen: ids_seen.clone(),
                    };
                    let payload = Payload::ReadOk { messages: ids_seen };
                    let response =
                        Message::new(node_id, request.src, msg_id, request.body.msg_id, payload);
                    response.send();
                    node
                }
                Payload::Gossip { ids_to_see } => {
                    let ids_not_seen_by_self: HashSet<usize> =
                        ids_to_see.difference(&ids_seen).copied().collect();
                    let ids_not_seen_by_other: HashSet<usize> =
                        ids_seen.difference(&ids_to_see).copied().collect();
                    ids_seen.extend(ids_not_seen_by_self);
                    let node = Node::Broadcasting {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        ids_seen,
                    };
                    if ids_not_seen_by_other.is_empty() {
                        node
                    } else {
                        let payload = Payload::GossipOk {
                            ids_to_see: ids_not_seen_by_other,
                        };
                        let response = Message::new(node_id, request.src, None, None, payload);
                        response.send();
                        node
                    }
                }
                payload => {
                    error!(target: "invalid payload", node_type="Broadcasting", payload = ?payload);
                    Node::Broadcasting {
                        msg_id,
                        node_id,
                        node_ids,
                        ids_seen,
                    }
                }
            },
            Node::Networked {
                msg_id,
                node_id,
                node_ids,
                mut ids_seen_by_neighbours,
            } => match request.body.payload {
                Payload::Broadcast { message } => {
                    let mut ids_seen = HashSet::new();
                    ids_seen.insert(message);
                    let node = Node::NetworkedBroadcasting {
                        msg_id: msg_id + 1,
                        node_id: node_id.clone(),
                        node_ids,
                        ids_seen,
                        ids_seen_by_neighbours,
                    };
                    let payload = Payload::BroadcastOk;
                    let response =
                        Message::new(node_id, request.src, msg_id, request.body.msg_id, payload);
                    response.send();
                    node
                }
                Payload::Read => {
                    let ids_seen = HashSet::new();
                    let node = Node::NetworkedBroadcasting {
                        msg_id: msg_id + 1,
                        node_id: node_id.clone(),
                        node_ids,
                        ids_seen: ids_seen.clone(),
                        ids_seen_by_neighbours,
                    };
                    let payload = Payload::ReadOk { messages: ids_seen };
                    let response =
                        Message::new(node_id, request.src, msg_id, request.body.msg_id, payload);
                    response.send();
                    node
                }
                Payload::Gossip { ids_to_see } => {
                    ids_seen_by_neighbours.update(request.src.clone(), ids_to_see.clone());
                    let node = Node::NetworkedBroadcasting {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        ids_seen: ids_to_see.clone(),
                        ids_seen_by_neighbours,
                    };
                    let payload = Payload::GossipOk { ids_to_see };
                    let response =
                        Message::new(node_id, request.src, msg_id, request.body.msg_id, payload);
                    response.send();
                    node
                }

                payload => {
                    error!(target: "invalid payload", node_type="Networked", payload = ?payload);
                    Node::Networked {
                        msg_id,
                        node_id,
                        node_ids,
                        ids_seen_by_neighbours,
                    }
                }
            },
            Node::NetworkedBroadcasting {
                msg_id,
                node_id,
                node_ids,
                mut ids_seen,
                mut ids_seen_by_neighbours,
            } => match request.body.payload {
                Payload::Broadcast { message } => {
                    ids_seen.insert(message);
                    let node = Node::NetworkedBroadcasting {
                        msg_id: msg_id + 1,
                        node_id: node_id.clone(),
                        node_ids,
                        ids_seen,
                        ids_seen_by_neighbours,
                    };
                    let payload = Payload::BroadcastOk;
                    let response =
                        Message::new(node_id, request.src, msg_id, request.body.msg_id, payload);
                    response.send();
                    node
                }
                Payload::Read => {
                    let node = Node::NetworkedBroadcasting {
                        msg_id: msg_id + 1,
                        node_id: node_id.clone(),
                        node_ids,
                        ids_seen: ids_seen.clone(),
                        ids_seen_by_neighbours,
                    };
                    let payload = Payload::ReadOk { messages: ids_seen };
                    let response =
                        Message::new(node_id, request.src, msg_id, request.body.msg_id, payload);
                    response.send();
                    node
                }
                Payload::Gossip { ids_to_see } => {
                    let id_number = node_id.id_number();
                    let duration = Duration::from_micros(id_number as u64);
                    thread::sleep(duration);
                    let ids_not_seen_by_self: HashSet<usize> =
                        ids_to_see.difference(&ids_seen).copied().collect();
                    let ids_not_seen_by_other: HashSet<usize> =
                        ids_seen.difference(&ids_to_see).copied().collect();
                    ids_seen.extend(ids_not_seen_by_self);
                    let node = Node::NetworkedBroadcasting {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        ids_seen,
                        ids_seen_by_neighbours,
                    };
                    if ids_not_seen_by_other.is_empty() {
                        node
                    } else {
                        let payload = Payload::GossipOk {
                            ids_to_see: ids_not_seen_by_other,
                        };
                        let response = Message::new(node_id, request.src, None, None, payload);
                        response.send();
                        node
                    }
                }
                Payload::GossipOk { ids_to_see } => {
                    ids_seen.extend(ids_to_see.clone());
                    ids_seen_by_neighbours.update(request.src, ids_to_see);
                    Node::NetworkedBroadcasting {
                        msg_id,
                        node_id: node_id.clone(),
                        node_ids,
                        ids_seen,
                        ids_seen_by_neighbours,
                    }
                }
                payload => {
                    error!(target: "invalid payload", node_type="NetworkedBroadcasting", payload = ?payload);
                    Node::NetworkedBroadcasting {
                        msg_id,
                        node_id,
                        node_ids,
                        ids_seen,
                        ids_seen_by_neighbours,
                    }
                }
            },
        }
    }

    pub fn gossip(&self) {
        if let Node::NetworkedBroadcasting {
            msg_id: _,
            node_id,
            node_ids: _,
            ids_seen,
            ids_seen_by_neighbours,
        } = self
        {
            let id_number = node_id.id_number();
            let duration = Duration::from_micros(id_number as u64);
            thread::sleep(duration);
            ids_seen_by_neighbours
                .0
                .iter()
                .for_each(|(neighbour, ids_seen_by_neighbour)| {
                    let ids_to_see: HashSet<usize> = ids_seen
                        .difference(ids_seen_by_neighbour)
                        .copied()
                        .collect();
                    if !ids_to_see.is_empty() {
                        let payload = Payload::Gossip { ids_to_see };
                        let request =
                            Message::new(node_id.clone(), neighbour.clone(), None, None, payload);
                        request.send();
                    }
                })
        }
    }
}

fn handle_init_request(
    msg_id: usize,
    node_id: NodeId,
    node_ids: HashSet<NodeId>,
    dest: NodeId,
    in_reply_to: impl Into<Option<usize>>,
) -> Node {
    let node = Node::Initialised {
        msg_id: msg_id + 1,
        node_id: node_id.clone(),
        node_ids,
    };
    let response_payload = Payload::InitOk;
    let response = Message::new(node_id, dest, msg_id, in_reply_to, response_payload);
    response.send();
    node
}

fn handle_echo_request(
    msg_id: usize,
    node_id: NodeId,
    node_ids: HashSet<NodeId>,
    echo: String,
    dest: NodeId,
    in_reply_to: impl Into<Option<usize>>,
) -> Node {
    let node = Node::Initialised {
        msg_id: msg_id + 1,
        node_id: node_id.clone(),
        node_ids,
    };
    let response_payload = Payload::EchoOk { echo };
    let response = Message::new(node_id, dest, msg_id, in_reply_to, response_payload);
    response.send();
    node
}

fn handle_generate_request(
    msg_id: usize,
    node_id: NodeId,
    node_ids: HashSet<NodeId>,
    dest: NodeId,
    in_reply_to: impl Into<Option<usize>>,
) -> Node {
    let node = Node::Initialised {
        msg_id: msg_id + 1,
        node_id: node_id.clone(),
        node_ids,
    };
    let id = Uuid::new_v4();
    let response_payload = Payload::GenerateOk { id };
    let response = Message::new(node_id, dest, msg_id, in_reply_to, response_payload);
    response.send();
    node
}

fn handle_topology_request(
    msg_id: usize,
    node_id: NodeId,
    node_ids: HashSet<NodeId>,
    _: HashMap<NodeId, HashSet<NodeId>>,
    dest: NodeId,
    in_reply_to: impl Into<Option<usize>>,
) -> Node {
    let index = node_id.id_number();
    let neighbours = match node_id.is_hub_node() {
        true => {
            let mut spoke_nodes = node_ids.clone();
            spoke_nodes.retain(|node_id| !node_id.is_hub_node());
            spoke_nodes
        }
        false => {
            let mut neighbours = HashSet::new();
            let maximum_index = node_ids.len() - 1;
            let nextdoor_neighbour_index = if index == maximum_index { 1 } else { index + 1 };
            let nextdoor_neighbour = NodeId::new(nextdoor_neighbour_index);
            neighbours.insert(nextdoor_neighbour);
            let mut hub_nodes = node_ids.clone();
            hub_nodes.retain(|node_id| node_id.is_hub_node());
            neighbours.extend(hub_nodes);
            neighbours
        }
    };
    info!(target: "Neighbours", neighbours = ?neighbours);
    let ids_seen_by_neighbours = IdsSeenByNeighbours::new(neighbours);
    let node = Node::Networked {
        msg_id: msg_id + 1,
        node_id: node_id.clone(),
        node_ids,
        ids_seen_by_neighbours,
    };
    let response_payload = Payload::TopologyOk;
    let response = Message::new(node_id, dest, msg_id, in_reply_to, response_payload);
    response.send();
    node
}

#[derive(Debug)]
pub struct IdsSeenByNeighbours(HashMap<NodeId, HashSet<usize>>);

impl IdsSeenByNeighbours {
    fn new(neighbours: HashSet<NodeId>) -> Self {
        let mut ids_seen_by_neighbours = HashMap::new();
        for neighbour in neighbours {
            let ids_seen = HashSet::new();
            ids_seen_by_neighbours.insert(neighbour, ids_seen);
        }
        Self(ids_seen_by_neighbours)
    }
    fn update(&mut self, neighbour: NodeId, ids_seen: HashSet<usize>) {
        if let Some(ids_seen_by_neighbour) = self.0.get_mut(&neighbour) {
            ids_seen_by_neighbour.extend(ids_seen);
        }
    }
}
