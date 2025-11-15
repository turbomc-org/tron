use chrono::{TimeZone, Utc};

pub mod error;
pub mod format_message;
pub mod math;
pub mod message;
pub mod mongodb;
pub mod name_generator;
pub mod redis;
pub mod serde;
pub mod templates;

pub fn is_expired(saved_timestamp: u64) -> bool {
    let now = Utc::now().timestamp() as u64;
    now > saved_timestamp
}

pub fn format_timestamp_indian_style(timestamp: u64) -> String {
    let datetime_utc = Utc.timestamp_opt(timestamp as i64, 0).unwrap();
    let datetime_ist = datetime_utc
        .with_timezone(&chrono::offset::FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap());

    datetime_ist.format("%d/%m/%Y %H:%M").to_string()
}
