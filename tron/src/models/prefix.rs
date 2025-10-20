use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Prefix {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "prefixText")]
    pub prefix_text: String,
    #[serde(rename = "displayColorHex")]
    pub display_color_hex: String,
    pub price: u32,
}
