use serde::{Deserialize, Serialize};

use super::{LogEntry, LogMessage, LogOffset};

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Hash, Serialize, Clone)]
pub struct LogEntries(Vec<LogEntry>);

impl LogEntries {
    // pub fn new(log_entries: Vec<LogEntry>) -> Self {
    //     Self(log_entries)
    // }

    pub fn iter(&self) -> impl Iterator<Item = &LogEntry> {
        self.0.iter()
    }

    pub fn max_log_entry(&self) -> Option<&LogEntry> {
        self.iter()
            .reduce(|previous, current| previous.max(current))
    }

    pub fn append_message(&mut self, message: LogMessage) -> LogOffset {
        let log_offset = match self.max_log_entry() {
            None => LogOffset::default(),
            Some(max_log_entry) => max_log_entry.increment_offset(),
        };
        let log_entry = LogEntry::new(log_offset.clone(), message);
        self.append_entry(log_entry);
        log_offset
    }

    pub fn append_entry(&mut self, log_entry: LogEntry) {
        self.0.push(log_entry);
    }

    pub fn append_entries(&mut self, log_entries: LogEntries) {
        self.0.extend(log_entries.iter().cloned());
    }

    #[allow(dead_code)]
    pub fn insert_message(&mut self, offset: LogOffset, message: LogMessage) {
        let log_entry = LogEntry::new(offset, message);
        self.insert_entry(log_entry);
    }

    #[allow(dead_code)]
    pub fn insert_entry(&mut self, entry: LogEntry) {
        self.append_entry(entry);
        self.0.sort();
    }

    pub fn since_offset(&self, offset: &LogOffset) -> Self {
        self.iter()
            .skip_while(|log_entry| log_entry.offset() < offset)
            .cloned()
            .collect()
    }
}

impl Default for LogEntries {
    fn default() -> Self {
        let log_entries = Vec::new();
        Self(log_entries)
    }
}

impl FromIterator<LogEntry> for LogEntries {
    fn from_iter<T: IntoIterator<Item = LogEntry>>(iter: T) -> Self {
        let mut log_entries = LogEntries::default();
        iter.into_iter()
            .for_each(|log_entry| log_entries.append_entry(log_entry));
        log_entries
    }
}
