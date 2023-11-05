use std::{
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

use tracing::{error, info};

use crate::{
    log::Logs,
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
        log: Logs,
    },
}

impl Node {
    pub fn new() -> Self {
        Node::Uninitialised { msg_id: 0 }
    }

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
                mut log,
            } => match request.body.payload {
                Payload::Send { key, msg } => {
                    let offset = log.append_message(key, msg);
                    let node = Node::Initialised {
                        msg_id: msg_id + 1,
                        node_id: node_id.clone(),
                        node_ids,
                        log,
                    };
                    let response_payload = Payload::SendOk { offset };
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
                Payload::Poll { offsets } => {
                    let messages = log.since_offset(offsets).as_messages();
                    let node = Node::Initialised {
                        msg_id: msg_id + 1,
                        node_id: node_id.clone(),
                        node_ids,
                        log,
                    };
                    let response_payload = Payload::PollOk { msgs: messages };
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
                Payload::CommitOffsets { offsets } => {
                    log.commit_offsets(offsets);
                    let node = Node::Initialised {
                        msg_id: msg_id + 1,
                        node_id: node_id.clone(),
                        node_ids,
                        log,
                    };
                    let response_payload = Payload::CommitOffsetsOk;
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
                Payload::ListCommittedOffsets { keys } => {
                    let offsets = log.list_committed_offsets(keys);
                    let node = Node::Initialised {
                        msg_id: msg_id + 1,
                        node_id: node_id.clone(),
                        node_ids,
                        log,
                    };
                    let response_payload = Payload::ListCommittedOffsetsOk { offsets };
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
                payload => {
                    error!(target: "invalid payload", node_type = "Initialised", payload = ?payload);
                    Node::Initialised {
                        msg_id,
                        node_id,
                        node_ids,
                        log,
                    }
                }
            },
        }
    }

    // pub fn gossip(&self) {
    //     if let Node::NetworkedBroadcasting {
    //         msg_id: _,
    //         node_id,
    //         node_ids: _,
    //         ids_seen,
    //         ids_seen_by_neighbours,
    //     } = self
    //     {
    //         let id_number = node_id.id_number();
    //         let duration = Duration::from_micros(id_number as u64);
    //         thread::sleep(duration);
    //         ids_seen_by_neighbours
    //             .0
    //             .iter()
    //             .for_each(|(neighbour, ids_seen_by_neighbour)| {
    //                 let ids_to_see: HashSet<usize> = ids_seen
    //                     .difference(ids_seen_by_neighbour)
    //                     .copied()
    //                     .collect();
    //                 if !ids_to_see.is_empty() {
    //                     todo!()
    //                 }
    //             })
    //     }
    // }
}

fn handle_init_request(
    msg_id: usize,
    node_id: NodeId,
    node_ids: HashSet<NodeId>,
    dest: NodeId,
    in_reply_to: impl Into<Option<usize>>,
) -> Node {
    let log = Logs::default();
    let node = Node::Initialised {
        msg_id: msg_id + 1,
        node_id: node_id.clone(),
        node_ids,
        log,
    };
    let response_payload = Payload::InitOk;
    let response = Message::new(node_id, dest, msg_id, in_reply_to, response_payload);
    response.send();
    node
}

// fn handle_echo_request(
//     msg_id: usize,
//     node_id: NodeId,
//     node_ids: HashSet<NodeId>,
//     echo: String,
//     dest: NodeId,
//     in_reply_to: impl Into<Option<usize>>,
// ) -> Node {
//     let node = Node::Initialised {
//         msg_id: msg_id + 1,
//         node_id: node_id.clone(),
//         node_ids,
//     };
//     todo!()
// }

// fn handle_generate_request(
//     msg_id: usize,
//     node_id: NodeId,
//     node_ids: HashSet<NodeId>,
//     dest: NodeId,
//     in_reply_to: impl Into<Option<usize>>,
// ) -> Node {
//     let node = Node::Initialised {
//         msg_id: msg_id + 1,
//         node_id: node_id.clone(),
//         node_ids,
//     };
//     todo!()
// }

// fn handle_topology_request(
//     msg_id: usize,
//     node_id: NodeId,
//     node_ids: HashSet<NodeId>,
//     _: HashMap<NodeId, HashSet<NodeId>>,
//     dest: NodeId,
//     in_reply_to: impl Into<Option<usize>>,
// ) -> Node {
//     let index = node_id.id_number();
//     let neighbours = match node_id.is_hub_node() {
//         true => {
//             let mut spoke_nodes = node_ids.clone();
//             spoke_nodes.retain(|node_id| !node_id.is_hub_node());
//             spoke_nodes
//         }
//         false => {
//             let mut neighbours = HashSet::new();
//             let maximum_index = node_ids.len() - 1;
//             let nextdoor_neighbour_index = if index == maximum_index { 1 } else { index + 1 };
//             let nextdoor_neighbour = NodeId::new(nextdoor_neighbour_index);
//             neighbours.insert(nextdoor_neighbour);
//             let mut hub_nodes = node_ids.clone();
//             hub_nodes.retain(|node_id| node_id.is_hub_node());
//             neighbours.extend(hub_nodes);
//             neighbours
//         }
//     };
//     info!(target: "Neighbours", neighbours = ?neighbours);
//     let ids_seen_by_neighbours = IdsSeenByNeighbours::new(neighbours);
//     let node = Node::Networked {
//         msg_id: msg_id + 1,
//         node_id: node_id.clone(),
//         node_ids,
//         ids_seen_by_neighbours,
//     };
//     todo!()
// }

// #[derive(Debug)]
// pub struct IdsSeenByNeighbours(HashMap<NodeId, HashSet<usize>>);

// impl IdsSeenByNeighbours {
//     fn new(neighbours: HashSet<NodeId>) -> Self {
//         let mut ids_seen_by_neighbours = HashMap::new();
//         for neighbour in neighbours {
//             let ids_seen = HashSet::new();
//             ids_seen_by_neighbours.insert(neighbour, ids_seen);
//         }
//         Self(ids_seen_by_neighbours)
//     }
//     fn update(&mut self, neighbour: NodeId, ids_seen: HashSet<usize>) {
//         if let Some(ids_seen_by_neighbour) = self.0.get_mut(&neighbour) {
//             ids_seen_by_neighbour.extend(ids_seen);
//         }
//     }
// }
