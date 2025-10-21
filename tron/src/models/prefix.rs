use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Encode, Decode, Clone)]
pub struct Prefix {
    pub id: u64,
    pub prefix_text: String,
    pub display_color_hex: String,
    pub price: u32,
}
