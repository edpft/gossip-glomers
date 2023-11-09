use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Clone)]
pub struct LogOffset(usize);

impl LogOffset {
    pub fn increment(&self) -> Self {
        let current = self.0;
        let next = current + 1;
        Self(next)
    }
}

impl From<usize> for LogOffset {
    fn from(offset: usize) -> Self {
        Self(offset)
    }
}
