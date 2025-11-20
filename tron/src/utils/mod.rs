use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{TimeZone, Utc};

pub mod error;
pub mod format_message;
pub mod get;
pub mod join;
pub mod math;
pub mod message;
pub mod mongodb;
pub mod name_generator;
pub mod redis;
pub mod serde;
pub mod templates;
pub mod title;

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

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(122_880, 3, 4, None)?,
    );

    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(
    password: &str,
    hashed_password: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hashed_password)?;
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
