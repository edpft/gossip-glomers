use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Clone)]
pub struct LogKey(Box<str>);

impl From<&str> for LogKey {
    fn from(key: &str) -> Self {
        Self(key.into())
    }
}
