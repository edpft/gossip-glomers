use std::collections::{HashMap, HashSet};

use maelstrom_protocol::nodes::NodeId;

#[derive(Debug)]
pub struct IdsSeenByNeighbours(HashMap<NodeId, HashSet<usize>>);

impl IdsSeenByNeighbours {
    pub fn new(neighbours: HashSet<NodeId>) -> Self {
        let mut ids_seen_by_neighbours = HashMap::new();
        for neighbour in neighbours {
            let ids_seen = HashSet::new();
            ids_seen_by_neighbours.insert(neighbour, ids_seen);
        }
        Self(ids_seen_by_neighbours)
    }
    pub fn update(&mut self, neighbour: NodeId, ids_seen: HashSet<usize>) {
        if let Some(ids_seen_by_neighbour) = self.0.get_mut(&neighbour) {
            ids_seen_by_neighbour.extend(ids_seen);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&NodeId, &HashSet<usize>)> {
        self.0.iter()
    }
}
