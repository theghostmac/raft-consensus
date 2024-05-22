use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub term: u64,
    pub command: String, // can be a Vec<u8>
}

pub struct Log {
    pub(crate) entries: Vec<LogEntry>,
}

impl Log {
    pub fn new() -> Self {
        Self { entries: Vec::new()}
    }

    pub fn append(&mut self, entry: LogEntry) {
        self.entries.push(entry)
    }

    pub fn last_entry(&self) -> Option<&LogEntry> {
        self.entries.last()
    }

    pub fn last_index(&self) -> u64 {
        self.entries.len() as u64
    }

    pub fn last_term(&self) -> Option<u64> {
        self.entries.last().map(|entry| entry.term)
    }

    // TODO: add any other log management methods...
}
