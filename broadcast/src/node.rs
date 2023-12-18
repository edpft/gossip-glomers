use std::{collections::HashSet, thread, time::Duration};

use maelstrom_protocol::{
    gossip::Gossip,
    messages::{Message, MessageId},
    nodes::{Node, NodeId},
};
use tracing::{error, info};

use crate::{ids_seen_by_neighbours::IdsSeenByNeighbours, payload::BroadcastPayload};

#[derive(Debug)]
pub enum BroadcastNode {
    Uninitialised {
        msg_id: MessageId,
    },
    Initialised {
        msg_id: MessageId,
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
    },
    Networked {
        msg_id: MessageId,
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
        ids_seen_by_neighbours: IdsSeenByNeighbours,
    },
    Broadcasting {
        msg_id: MessageId,
        node_id: NodeId,
        node_ids: HashSet<NodeId>,
        ids_seen: HashSet<usize>,
        ids_seen_by_neighbours: IdsSeenByNeighbours,
    },
}

impl Node<BroadcastPayload> for BroadcastNode {
    fn new() -> Self {
        let msg_id = MessageId::default();
        BroadcastNode::Uninitialised { msg_id }
    }

    fn id(&self) -> Option<&NodeId> {
        match self {
            BroadcastNode::Uninitialised { .. } => None,
            BroadcastNode::Initialised {
                msg_id: _,
                node_id,
                node_ids: _,
            }
            | BroadcastNode::Networked {
                msg_id: _,
                node_id,
                node_ids: _,
                ids_seen_by_neighbours: _,
            }
            | BroadcastNode::Broadcasting {
                msg_id: _,
                node_id,
                node_ids: _,
                ids_seen: _,
                ids_seen_by_neighbours: _,
            } => Some(node_id),
        }
    }

    fn msg_id(&self) -> &MessageId {
        match self {
            BroadcastNode::Uninitialised { msg_id }
            | BroadcastNode::Initialised {
                msg_id,
                node_id: _,
                node_ids: _,
            }
            | BroadcastNode::Networked {
                msg_id,
                node_id: _,
                node_ids: _,
                ids_seen_by_neighbours: _,
            }
            | BroadcastNode::Broadcasting {
                msg_id,
                node_id: _,
                node_ids: _,
                ids_seen: _,
                ids_seen_by_neighbours: _,
            } => msg_id,
        }
    }

    fn handle(self, request: Message<BroadcastPayload>) -> Self {
        info!(target: "Received message", message = ?request);
        let (node, response_payload) = match self {
            BroadcastNode::Uninitialised { msg_id } => match &request.body.payload {
                BroadcastPayload::Init { node_id, node_ids } => {
                    let node = BroadcastNode::Initialised {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                    };
                    let response_payload = BroadcastPayload::InitOk;
                    (node, Some(response_payload))
                }
                unexpected_payload => {
                    error!(target: "invalid payload", node_type = "Uninitialised", payload = ?unexpected_payload);
                    let node = BroadcastNode::Uninitialised {
                        msg_id: msg_id.clone(),
                    };
                    (node, None)
                }
            },
            BroadcastNode::Initialised {
                msg_id,
                node_id,
                node_ids,
            } => match &request.body.payload {
                BroadcastPayload::Topology { .. } => {
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
                            let nextdoor_neighbour_index =
                                if index == maximum_index { 1 } else { index + 1 };
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
                    let node = BroadcastNode::Networked {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        ids_seen_by_neighbours,
                    };
                    let response_payload = BroadcastPayload::TopologyOk;
                    (node, Some(response_payload))
                }
                unexpected_payload => {
                    error!(target: "invalid payload", node_type = "Uninitialised", payload = ?unexpected_payload);
                    let node = BroadcastNode::Uninitialised {
                        msg_id: msg_id.clone(),
                    };
                    (node, None)
                }
            },
            BroadcastNode::Networked {
                msg_id,
                node_id,
                node_ids,
                mut ids_seen_by_neighbours,
            } => match &request.body.payload {
                BroadcastPayload::Broadcast { message } => {
                    let mut ids_seen = HashSet::new();
                    ids_seen.insert(message);
                    let node = BroadcastNode::Broadcasting {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        ids_seen: ids_seen.into_iter().cloned().collect(),
                        ids_seen_by_neighbours,
                    };
                    let payload = BroadcastPayload::BroadcastOk;
                    (node, Some(payload))
                }
                BroadcastPayload::Read => {
                    let ids_seen = HashSet::new();
                    let node = BroadcastNode::Broadcasting {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        ids_seen: ids_seen.clone(),
                        ids_seen_by_neighbours,
                    };
                    let payload = BroadcastPayload::ReadOk { messages: ids_seen };
                    (node, Some(payload))
                }
                BroadcastPayload::Gossip { ids_to_see } => {
                    ids_seen_by_neighbours.update(request.src.clone(), ids_to_see.clone());
                    let node = BroadcastNode::Broadcasting {
                        msg_id: msg_id.clone(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        ids_seen: ids_to_see.clone(),
                        ids_seen_by_neighbours,
                    };
                    let payload = BroadcastPayload::GossipOk {
                        ids_to_see: ids_to_see.clone(),
                    };
                    (node, Some(payload))
                }
                payload => {
                    error!(target: "invalid payload", node_type="Networked", payload = ?payload);
                    let node = BroadcastNode::Networked {
                        msg_id: msg_id.clone(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        ids_seen_by_neighbours,
                    };
                    (node, None)
                }
            },
            Self::Broadcasting {
                msg_id,
                node_id,
                node_ids,
                mut ids_seen,
                mut ids_seen_by_neighbours,
            } => match &request.body.payload {
                BroadcastPayload::Broadcast { message } => {
                    ids_seen.insert(*message);
                    let node = BroadcastNode::Broadcasting {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        ids_seen,
                        ids_seen_by_neighbours,
                    };
                    let payload = BroadcastPayload::BroadcastOk;
                    (node, Some(payload))
                }
                BroadcastPayload::Read => {
                    let node = BroadcastNode::Broadcasting {
                        msg_id: msg_id.increment(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        ids_seen: ids_seen.clone(),
                        ids_seen_by_neighbours,
                    };
                    let payload = BroadcastPayload::ReadOk { messages: ids_seen };
                    (node, Some(payload))
                }
                BroadcastPayload::Gossip { ids_to_see } => {
                    let id_number = node_id.id_number();
                    let duration = Duration::from_micros(id_number as u64);
                    thread::sleep(duration);
                    let ids_not_seen_by_self: HashSet<usize> =
                        ids_to_see.difference(&ids_seen).copied().collect();
                    let ids_not_seen_by_other: HashSet<usize> =
                        ids_seen.difference(ids_to_see).copied().collect();
                    ids_seen.extend(ids_not_seen_by_self);
                    let node = BroadcastNode::Broadcasting {
                        msg_id: msg_id.clone(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        ids_seen,
                        ids_seen_by_neighbours,
                    };
                    if ids_not_seen_by_other.is_empty() {
                        (node, None)
                    } else {
                        let payload = BroadcastPayload::GossipOk {
                            ids_to_see: ids_not_seen_by_other,
                        };
                        (node, Some(payload))
                    }
                }
                BroadcastPayload::GossipOk { ids_to_see } => {
                    ids_seen.extend(ids_to_see.clone());
                    ids_seen_by_neighbours.update(request.src().clone(), ids_to_see.clone());
                    let node = BroadcastNode::Broadcasting {
                        msg_id: msg_id.clone(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        ids_seen,
                        ids_seen_by_neighbours,
                    };
                    (node, None)
                }
                payload => {
                    error!(target: "invalid payload", node_type="Broadcasting", payload = ?payload);
                    let node = BroadcastNode::Broadcasting {
                        msg_id: msg_id.clone(),
                        node_id: node_id.clone(),
                        node_ids: node_ids.clone(),
                        ids_seen,
                        ids_seen_by_neighbours,
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

impl Gossip for BroadcastNode {
    fn gossip(&self) {
        if let BroadcastNode::Broadcasting {
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
                .iter()
                .for_each(|(neighbour, ids_seen_by_neighbour)| {
                    let ids_to_see: HashSet<usize> = ids_seen
                        .difference(ids_seen_by_neighbour)
                        .copied()
                        .collect();
                    if !ids_to_see.is_empty() {
                        let payload = BroadcastPayload::Gossip { ids_to_see };
                        let request =
                            Message::new(node_id.clone(), neighbour.clone(), None, None, payload);
                        request.send();
                    }
                })
        }
    }
}
