mod log;
mod log_entries;
mod log_entry;
mod log_key;
mod log_message;
mod log_offset;
mod logs;
mod messages;
mod offsets;

pub use log::Log;
pub use log_entries::LogEntries;
pub use log_entry::LogEntry;
pub use log_key::LogKey;
pub use log_message::LogMessage;
pub use log_offset::LogOffset;
pub use logs::Logs;
pub use messages::Messages;
pub use offsets::Offsets;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_append_and_insert() {
        let mut append_log = Logs::default();

        append_log.append_message("k0", 100);
        append_log.append_message("k0", 200);
        append_log.append_message("k0", 300);

        append_log.append_message("k1", 101);
        append_log.append_message("k1", 201);
        append_log.append_message("k1", 301);

        append_log.append_message("k2", 102);
        append_log.append_message("k2", 202);
        append_log.append_message("k2", 302);

        let mut insert_log = Logs::default();

        insert_log.insert_message("k0", 2, 300);
        insert_log.insert_message("k0", 1, 200);
        insert_log.insert_message("k0", 0, 100);

        insert_log.insert_message("k1", 0, 101);
        insert_log.insert_message("k1", 2, 301);
        insert_log.insert_message("k1", 1, 201);

        insert_log.insert_message("k2", 1, 202);
        insert_log.insert_message("k2", 0, 102);
        insert_log.insert_message("k2", 2, 302);

        assert_eq!(append_log, insert_log);
    }
}
