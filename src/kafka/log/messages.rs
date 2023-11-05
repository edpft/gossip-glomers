use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{LogEntries, LogKey};

#[derive(Debug, Default, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub struct Messages(HashMap<LogKey, LogEntries>);

impl Messages {
    pub fn insert_entries(&mut self, key: LogKey, entries: LogEntries) {
        self.0.insert(key, entries);
    }
}
