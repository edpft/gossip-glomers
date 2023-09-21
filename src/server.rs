use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::{
    error::Error,
    message::{Body, Message, Payload},
};

type MessageSeen = Option<HashSet<usize>>;

pub struct Server {
    node_id: String,
    msg_id: usize,
    ids_seen: MessageSeen,
    ids_seen_by_neighbours: Option<HashMap<String, MessageSeen>>,
}

impl Server {
    fn new(node_id: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            msg_id: 0,
            ids_seen: None,
            ids_seen_by_neighbours: None,
        }
    }

    pub fn handle_initial_request(&mut self, request: Message) -> color_eyre::Result<Message> {
        match request.body.payload {
            Payload::Init {
                node_id: _,
                node_ids: _,
            } => {
                let response_payload = Payload::InitOk;
                let response_body = Body::new(self.msg_id, request.body.msg_id, response_payload);
                let response = Message::new(&self.node_id, request.src, response_body);
                self.msg_id += 1;
                Ok(response)
            }
            _ => {
                let error = Error::Initialisation;
                Err(error)?
            }
        }
    }

    pub fn handle_request(&mut self, request: Message) -> Option<Message> {
        let response_payload = match request.body.payload {
            Payload::Init { .. } => None,
            Payload::InitOk => None,
            Payload::Echo { echo } => {
                let payload = Payload::EchoOk { echo };
                Some(payload)
            }
            Payload::EchoOk { .. } => None,
            Payload::Generate => {
                let id = Uuid::new_v4();
                let payload = Payload::GenerateOk { id };
                Some(payload)
            }
            Payload::GenerateOk { .. } => None,
            Payload::Broadcast { message } => {
                if self.ids_seen.is_none() {
                    let ids_seen = HashSet::new();
                    self.ids_seen = Some(ids_seen);
                };
                self.ids_seen
                    .as_mut()
                    .map(|ids_seen| ids_seen.insert(message));
                let payload = Payload::BroadcastOk;
                Some(payload)
            }
            Payload::BroadcastOk => None,
            Payload::Read => {
                let messages = self.ids_seen.clone();
                let payload = Payload::ReadOk { messages };
                Some(payload)
            }
            Payload::ReadOk { .. } => None,
            Payload::Topology { topology } => {
                if let Some(neighbours) = topology.get(&self.node_id) {
                    let ids_seen_by_neighbours: HashMap<String, MessageSeen> = neighbours
                        .iter()
                        .map(|neighbour| (neighbour.clone(), None))
                        .collect();
                    self.ids_seen_by_neighbours = Some(ids_seen_by_neighbours);
                }
                let payload = Payload::TopologyOk;
                Some(payload)
            }
            Payload::TopologyOk => None,
            Payload::Gossip { ids_to_see } => {
                let neighbour = request.src.clone();
                let ids_seen_by_neighbour = &ids_to_see;
                self.update_ids_seen_by_neighbours(&neighbour, ids_seen_by_neighbour);
                self.update_ids_seen(ids_seen_by_neighbour);
                let ids_to_gossip = get_ids_to_gossip(&self.ids_seen, ids_seen_by_neighbour);
                let payload = Payload::GossipOk {
                    ids_seen: ids_to_gossip,
                };
                Some(payload)
            }
            Payload::GossipOk { ids_seen } => {
                let neighbour = request.src.clone();
                let ids_seen_by_neighbour = &ids_seen;
                self.update_ids_seen_by_neighbours(&neighbour, ids_seen_by_neighbour);
                None
            }
        };
        match response_payload {
            None => None,
            Some(response_payload) => match response_payload {
                Payload::GossipOk { .. } => {
                    let response_body = Body::new(None, None, response_payload);
                    let response = Message::new(&self.node_id, request.src, response_body);
                    Some(response)
                }
                _ => {
                    let response_body =
                        Body::new(self.msg_id, request.body.msg_id, response_payload);
                    let response = Message::new(&self.node_id, request.src, response_body);
                    self.msg_id += 1;
                    Some(response)
                }
            },
        }
    }

    pub fn generate_gossip(&mut self) -> Option<Vec<Message>> {
        match &self.ids_seen_by_neighbours {
            Some(ids_seen_by_neighbours) => {
                let messages_to_gossip: Vec<Message> = ids_seen_by_neighbours
                    .iter()
                    .map(|(neighbour, ids_seen_by_neighbour)| {
                        let ids_to_gossip =
                            get_ids_to_gossip(&self.ids_seen, ids_seen_by_neighbour);
                        let request_payload = Payload::Gossip {
                            ids_to_see: ids_to_gossip,
                        };
                        let request_body = Body::new(self.msg_id, None, request_payload);
                        Message::new(&self.node_id, neighbour, request_body)
                    })
                    .collect();
                Some(messages_to_gossip)
            }
            None => None,
        }
    }

    fn update_ids_seen_by_neighbours(
        &mut self,
        neighbour: &str,
        ids_seen_by_neighbour: &MessageSeen,
    ) {
        match (self.ids_seen_by_neighbours.as_mut(), ids_seen_by_neighbour) {
            (None, None) => (),
            (Some(_), None) => (),
            (None, Some(ids_seen_by_neighbour)) => {
                let mut ids_seen_by_neighbours = HashMap::new();
                let ids_seen_by_neighbour = ids_seen_by_neighbour.clone();
                let neighbour = neighbour.to_string();
                ids_seen_by_neighbours.insert(neighbour, Some(ids_seen_by_neighbour));
                self.ids_seen_by_neighbours = Some(ids_seen_by_neighbours);
            }
            (Some(ids_seen_by_neighbours), Some(ids_seen_by_neighbour)) => {
                if let Some(previous_ids_seen_by_neighbour) =
                    ids_seen_by_neighbours.get_mut(neighbour)
                {
                    match previous_ids_seen_by_neighbour {
                        Some(previous_ids_seen_by_neighbour) => {
                            previous_ids_seen_by_neighbour.extend(ids_seen_by_neighbour)
                        }
                        None => {
                            let neighbour = neighbour.to_string();
                            let ids_seen_by_neighbour = ids_seen_by_neighbour.clone();
                            ids_seen_by_neighbours.insert(neighbour, Some(ids_seen_by_neighbour));
                        }
                    }
                };
            }
        };
    }

    fn update_ids_seen(&mut self, ids_seen_by_neighbour: &MessageSeen) {
        match (&self.ids_seen, ids_seen_by_neighbour) {
            // If neither of us have seen anything or if I've seen something but you haven't seen anything, there's nothing to do
            (None, None) | (Some(_), None) => (),
            // If I haven't seen anything but you have, I update what I've seen
            (None, Some(ids_seen_by_neighbour)) => {
                let ids_seen_by_neighbour = ids_seen_by_neighbour.clone();
                self.ids_seen = Some(ids_seen_by_neighbour);
            }
            // If we've both seen something, I add what you've seen to what I've seen
            (Some(ids_seen), Some(ids_to_see)) => {
                let union: HashSet<usize> = ids_seen.union(ids_to_see).copied().collect();
                self.ids_seen = Some(union);
            }
        };
    }
}

fn get_ids_to_gossip(ids_seen: &MessageSeen, ids_seen_by_neighbour: &MessageSeen) -> MessageSeen {
    match (ids_seen, ids_seen_by_neighbour) {
        (Some(ids_seen), Some(ids_seen_by_neighbour)) => {
            let ids_to_gossip: HashSet<usize> = ids_seen
                .difference(ids_seen_by_neighbour)
                .copied()
                .collect();
            Some(ids_to_gossip)
        }
        (None, Some(_)) => None,
        (Some(ids_seen), None) => {
            let ids_to_gossip = ids_seen.clone();
            Some(ids_to_gossip)
        }
        (None, None) => None,
    }
}

impl TryFrom<&Message> for Server {
    type Error = Error;

    fn try_from(request: &Message) -> Result<Self, Self::Error> {
        match &request.body.payload {
            Payload::Init {
                node_id,
                node_ids: _,
            } => {
                let server = Server::new(node_id);
                Ok(server)
            }
            _ => {
                let error = Error::Initialisation;
                Err(error)?
            }
        }
    }
}
