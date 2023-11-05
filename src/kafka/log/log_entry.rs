use std::cmp::Ordering;

use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use super::{LogMessage, LogOffset};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize_tuple, Serialize_tuple)]
pub struct LogEntry {
    offset: LogOffset,
    message: LogMessage,
}

impl LogEntry {
    pub fn new(offset: LogOffset, message: LogMessage) -> Self {
        Self { offset, message }
    }

    pub fn increment_offset(&self) -> LogOffset {
        self.offset.increment()
    }

    pub fn offset(&self) -> &LogOffset {
        &self.offset
    }
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.offset.partial_cmp(&other.offset)
    }
}

impl Ord for LogEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset.cmp(&other.offset)
    }
}
