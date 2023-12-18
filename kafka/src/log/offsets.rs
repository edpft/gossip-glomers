use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{LogKey, LogOffset};

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
pub struct Offsets(HashMap<LogKey, Option<LogOffset>>);

impl Offsets {
    pub fn items(&mut self) -> impl Iterator<Item = (&LogKey, Option<&LogOffset>)> {
        self.0.iter().map(|(key, offset)| match offset {
            Some(offset) => (key, Some(offset)),
            None => (key, None),
        })
    }

    pub fn insert_offset(&mut self, key: LogKey, offset: Option<LogOffset>) {
        self.0.insert(key, offset);
    }
}
