use bincode::{Decode, Encode};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::GENERATOR;

#[derive(Serialize, Deserialize, Encode, Decode, Clone)]
pub struct Prefix {
    #[serde(rename = "_id")]
    pub id: u64,
    pub text: String,
    pub color: String,
    pub price: u64,
}

impl Prefix {
    pub fn new(text: String, color: String, price: u64) -> Self {
        Self {
            id: GENERATOR.generate(),
            text,
            color: color,
            price,
        }
    }
}
