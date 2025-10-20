use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::error::Error as MongoError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    edition: Edition,
    pub coins: i64,
    #[serde(rename = "playerName")]
    pub player_name: String,
    prefixes: Option<Vec<String>>,
    #[serde(rename = "selectedPrefix")]
    selected_prefix: Option<String>,
    #[serde(rename = "teamId")]
    team_id: Option<String>,
    friends: Vec<String>,
    #[serde(rename = "pendingIncomingRequests")]
    pending_incoming_request: Vec<String>,
    #[serde(rename = "pendingOutgoingRequests")]
    pending_outgoining_request: Vec<String>,
    #[serde(rename = "lastUpdated")]
    last_update: i64,
    #[serde(rename = "invitesBlocked")]
    invite_blocked: bool,
    #[serde(rename = "registeredIP")]
    pub registered_ip: Option<String>,
    #[serde(rename = "registrationTime")]
    registeration_time: i64,
    kills: i64,
    deaths: i64,
    #[serde(rename = "blockedPlayersFromFriendRequests")]
    pub blocked_players_from_friend_requests: Vec<String>,
    #[serde(rename = "vaultCount")]
    vault_count: i64,
    #[serde(rename = "ownedVaultIds")]
    owned_vault_ids: Vec<String>,
    #[serde(rename = "redeemedCodes")]
    redeemed_codes: Vec<String>,
    #[serde(rename = "discordId", default)]
    discord_id: Option<String>,
}

impl Player {
    pub fn coins(&self) -> i64 {
        self.coins
    }

    pub fn player_name(&self) -> &str {
        &self.player_name
    }

    pub fn kills(&self) -> i64 {
        self.kills
    }

    pub fn deaths(&self) -> i64 {
        self.deaths
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Edition {
    Java,
    Bedrock,
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

    pub async fn inc_coins(
        id: String,
        coins: i64,
        col: Collection<Self>,
    ) -> Result<mongodb::results::UpdateResult, IncCoinsError> {
        let player = col
            .find_one(doc! { "discordId": &id })
            .await
            .map_err(IncCoinsError::MongoError)?
            .ok_or(IncCoinsError::PlayerNotFound)?;

        if coins < 0 {
            let current_coins = player.coins;
            if current_coins < coins.abs() {
                return Err(IncCoinsError::InsufficientCoins);
            }
        }

        let result = col
            .update_one(
                doc! { "discordId": id },
                doc! {
                    "$inc": { "coins": coins },
                    "$set": { "coins": { "$max": [ { "$add": ["$coins", coins] }, 0 ] } }
                },
            )
            .await
            .map_err(IncCoinsError::MongoError)?;

        Ok(result)
    }

    pub async fn transfer(
        sender_id: String,
        recipient_id: String,
        amount: i64,
        col: Collection<Self>,
    ) -> Result<(), String> {
        let sender = match Self::find_by_discord(sender_id.clone(), col.clone()).await {
            Ok(player) => player,
            Err(_) => return Err("Sender not found".to_string()),
        };
        if sender.coins < amount {
            return Err("Insufficient coins".to_string());
        }
        if let Err(_) = Self::find_by_discord(recipient_id.clone(), col.clone()).await {
            return Err("Recipient not found".to_string());
        }
        if let Err(e) = Self::inc_coins(sender_id, -amount, col.clone()).await {
            return Err(format!("Failed to decrement sender's coins: {:?}", e));
        }
        if let Err(e) = Self::inc_coins(recipient_id, amount, col).await {
            return Err(format!("Failed to increment recipient's coins: {:?}", e));
        }
        Ok(())
    }
}
