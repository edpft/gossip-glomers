use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Clone)]
pub struct MessageId(usize);

impl MessageId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn increment(&self) -> Self {
        let current = self.0;
        let new = current + 1;
        Self::new(new)
    }
}
