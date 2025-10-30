use crate::bridge::PartialPrefix as CompiledPrefix;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::GENERATOR;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn decompile(complied_prefix: CompiledPrefix) -> Self {
        Self {
            id: GENERATOR.generate(),
            text: complied_prefix.text.clone(),
            color: complied_prefix.color.clone(),
            price: complied_prefix.price.clone(),
        }
    }

    pub fn compile(&self) -> CompiledPrefix {
        CompiledPrefix {
            text: self.text.clone(),
            color: self.color.clone(),
            price: self.price,
        }
    }
}
