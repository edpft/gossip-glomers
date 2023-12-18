use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Clone)]
pub struct NodeId(Box<str>);

impl NodeId {
    pub fn new(id: usize) -> Self {
        let id = format!("n{}", id);
        Self(id.into())
    }
    pub fn id_number(&self) -> usize {
        self.0
            .chars()
            .nth(1)
            .expect("There will always be 2 characters")
            .to_digit(10)
            .expect("The second character will always be a digit") as usize
    }

    pub fn is_hub_node(&self) -> bool {
        let id_number = self.id_number();
        id_number == 0
    }
}

impl From<usize> for NodeId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}
