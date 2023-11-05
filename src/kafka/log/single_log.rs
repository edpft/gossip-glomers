use serde::{Deserialize, Serialize};

use super::{LogEntries, LogMessage, LogOffset};

#[derive(Debug, Default, Deserialize, PartialEq, Eq, PartialOrd, Hash, Serialize, Clone)]
pub struct Log {
    entries: LogEntries,
    committed_offset: Option<LogOffset>,
}

impl Log {
    pub fn append_message(&mut self, message: LogMessage) -> LogOffset {
        self.entries.append_message(message)
    }

    // pub fn append_entry(&mut self, entry: LogEntry) {
    //     self.entries.append_entry(entry)
    // }

    pub fn append_entries(&mut self, entries: LogEntries) {
        self.entries.append_entries(entries)
    }

    // pub fn insert_message(&mut self, offset: LogOffset, message: LogMessage) {
    //     self.entries.insert_message(offset, message)
    // }

    pub fn since_offset(&self, offset: &LogOffset) -> LogEntries {
        self.entries.since_offset(offset)
    }

    pub fn commit_offset(&mut self, offset: Option<LogOffset>) {
        self.committed_offset = offset
    }

    pub fn entries(&self) -> &LogEntries {
        &self.entries
    }

    pub fn committed_offset(&self) -> Option<&LogOffset> {
        self.committed_offset.as_ref()
    }
}
