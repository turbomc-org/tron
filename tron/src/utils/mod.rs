use chrono::Utc;

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
