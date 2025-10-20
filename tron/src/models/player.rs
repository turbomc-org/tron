use crate::bridge::player_join_request::Edition as GrpcEdition;
use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::error::Error as MongoError;
use serde::{Deserialize, Serialize};

use crate::GENERATOR;

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
pub struct Player {
    pub id: u64,
    pub username: String,
    pub discord_id: Option<String>,
    pub edition: Edition,
    pub coins: u64,
    pub prefixes: Vec<String>,
    pub selected_prefix: Option<String>,
    pub team_id: Option<String>,
    pub friends: Vec<String>,
    pub invite_blocked: bool,
    pub kills: u64,
    pub deaths: u64,
    pub vault_count: u64,
    pub owned_vault_ids: Vec<String>,
    pub redeemed_codes: Vec<String>,
    pub blocked_requests: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Player {
    pub fn coins(&self) -> u64 {
        self.coins
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn kills(&self) -> u64 {
        self.kills
    }

    pub fn deaths(&self) -> u64 {
        self.deaths
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
#[serde(rename_all = "UPPERCASE")]
pub enum Edition {
    Java,
    Bedrock,
}

impl From<GrpcEdition> for Edition {
    fn from(value: GrpcEdition) -> Self {
        match value {
            GrpcEdition::Java => Edition::Java,
            GrpcEdition::Bedrock => Edition::Bedrock,
        }
    }
}

pub enum FindByDiscordErrors {
    NotFound,
    InternalError,
}

#[derive(Debug)]
pub enum IncCoinsError {
    PlayerNotFound,
    InsufficientCoins,
    MongoError(MongoError),
}

impl Player {
    pub fn new(username: String, edition: Edition) -> Self {
        let now: DateTime<Utc> = Utc::now();

        Self {
            id: GENERATOR.generate(),
            username,
            edition,
            discord_id: None,
            coins: 0,
            kills: 0,
            deaths: 0,
            prefixes: Vec::new(),
            selected_prefix: None,
            team_id: None,
            friends: Vec::new(),
            invite_blocked: false,
            vault_count: 0,
            owned_vault_ids: Vec::new(),
            redeemed_codes: Vec::new(),
            blocked_requests: Vec::new(),
            created_at: now.timestamp() as u64,
            updated_at: now.timestamp() as u64,
        }
    }

    pub async fn insert(&self, col: &Collection<Self>) -> anyhow::Result<()> {
        col.insert_one(self.clone()).await?;
        Ok(())
    }

    pub async fn find_by_username(
        username: String,
        col: &Collection<Self>,
    ) -> anyhow::Result<Option<Self>> {
        col.find_one(doc! {
            "username": username
        })
        .await
        .map_err(|_| anyhow::anyhow!("Failed to find player by username"))
    }

    pub async fn find_by_discord(
        id: String,
        col: Collection<Self>,
    ) -> Result<Self, FindByDiscordErrors> {
        match col
            .find_one(doc! {
                "discordId": id
            })
            .await
        {
            Ok(Some(player)) => Ok(player),
            Ok(None) => Err(FindByDiscordErrors::NotFound),
            Err(_) => Err(FindByDiscordErrors::InternalError),
        }
    }

    pub async fn inc_coins(id: u64, coins: i64, col: &Collection<Self>) -> anyhow::Result<()> {
        col.update_one(
            doc! { "_id": id as i64 },
            doc! {
                "$inc": { "coins": coins },
                "$set": { "coins": { "$max": [ { "$add": ["$coins", coins] }, 0 ] } }
            },
        )
        .await?;

        Ok(())
    }

    pub async fn transfer_coins(
        from_username: u64,
        to_id: u64,
        amount: u64,
        col: &Collection<Self>,
    ) -> anyhow::Result<()> {
        Self::inc_coins(from_id, -(amount as i64), col).await?;
        Self::inc_coins(to_id, amount as i64, col).await?;

        Ok(())
    }
}
