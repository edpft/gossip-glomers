use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Clone)]
pub struct LogMessage(i64);

impl From<i64> for LogMessage {
    fn from(message: i64) -> Self {
        Self(message)
    }
}
