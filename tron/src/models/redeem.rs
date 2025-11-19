use serde::{Deserialize, Serialize};

use crate::{GENERATOR, models::player::Rank};
use chrono::Utc;
use tron_protos::RedeemType;
use tron_protos::redeem_type;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Redeem {
    #[serde(rename = "_id")]
    pub id: u64,
    pub reward: Reward,
    pub code: String,
    pub expires_at: u64,
    pub created_by: u64,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Reward {
    Coins(u64),
    Rank(Rank),
}

impl Redeem {
    pub fn new(code: String, expires_at: u64, created_by: u64, reward: Reward) -> Self {
        let now = Utc::now().timestamp() as u64;
        Self {
            id: GENERATOR.generate(),
            reward,
            code,
            expires_at,
            created_by,
            created_at: now,
        }
    }
}

impl From<RedeemType> for Reward {
    fn from(proto: RedeemType) -> Self {
        match proto.kind {
            Some(redeem_type::Kind::Coins(coins)) => Reward::Coins(coins),
            Some(redeem_type::Kind::Rank(rank_i32)) => {
                Reward::Rank(Rank::try_from(rank_i32).unwrap_or(Rank::default()))
            }
            None => Reward::Coins(0),
        }
    }
}

impl From<Reward> for RedeemType {
    fn from(reward: Reward) -> Self {
        match reward {
            Reward::Coins(coins) => RedeemType {
                kind: Some(redeem_type::Kind::Coins(coins)),
            },
            Reward::Rank(rank) => RedeemType {
                kind: Some(redeem_type::Kind::Rank(rank as i32)),
            },
        }
    }
}
