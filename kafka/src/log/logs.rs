use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Log, LogEntries, LogKey, LogMessage, LogOffset, Messages, Offsets};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Logs(HashMap<LogKey, Log>);

impl Default for Logs {
    fn default() -> Self {
        let map = HashMap::new();
        Self(map)
    }
}

impl Logs {
    pub fn append_message(
        &mut self,
        key: impl Into<LogKey>,
        message: impl Into<LogMessage>,
    ) -> LogOffset {
        let key = key.into();
        match self.0.get_mut(&key) {
            Some(log) => log.append_message(message.into()),
            None => {
                let mut log = Log::default();
                let log_offset = log.append_message(message.into());
                self.0.insert(key, log);
                log_offset
            }
        }
    }

    pub fn append_entries(&mut self, key: impl Into<LogKey>, additional_entries: LogEntries) {
        let key = key.into();
        match self.0.get_mut(&key) {
            Some(log) => log.append_entries(additional_entries),
            None => {
                let mut log = Log::default();
                log.append_entries(additional_entries);
                self.0.insert(key, log);
            }
        }
    }

    #[allow(dead_code)]
    pub fn insert_message(
        &mut self,
        key: impl Into<LogKey>,
        offset: impl Into<LogOffset>,
        message: impl Into<LogMessage>,
    ) -> Option<()> {
        let key = key.into();
        match self.0.get_mut(&key) {
            Some(log) => log.insert_message(offset.into(), message.into()),
            None => {
                let mut log = Log::default();
                log.insert_message(offset.into(), message.into());
                self.0.insert(key, log);
            }
        }
        Some(())
    }

    pub fn since_offset(&mut self, mut offsets: Offsets) -> Self {
        let mut logs = Logs::default();
        offsets.items().for_each(|(key, offset)| {
            if let (Some(log), Some(offset)) = (self.0.get_mut(key), offset) {
                let log_entries_since_offset = log.since_offset(offset);
                logs.append_entries(key.clone(), log_entries_since_offset);
            }
        });
        logs
    }

    pub fn commit_offsets(&mut self, mut offsets: Offsets) {
        offsets.items().for_each(|(key, offset)| {
            if let Some(log) = self.0.get_mut(key) {
                let offset = offset.cloned();
                log.commit_offset(offset)
            }
        });
    }

    pub fn list_committed_offsets(&mut self, keys: Vec<LogKey>) -> Offsets {
        let mut offsets = Offsets::default();
        keys.iter().for_each(|key| {
            let Some(log) = self.0.get(key) else {
                offsets.insert_offset(key.clone(), Some(LogOffset::default()));
                return;
            };
            let Some(offset) = log.committed_offset().cloned() else {
                offsets.insert_offset(key.clone(), Some(LogOffset::default()));
                return;
            };
            offsets.insert_offset(key.clone(), Some(offset))
        });
        offsets
    }

    pub fn as_messages(&self) -> Messages {
        let mut messages = Messages::default();
        self.0.iter().for_each(|(key, log)| {
            let entries = log.entries().clone();
            messages.insert_entries(key.clone(), entries)
        });
        messages
    }
}
