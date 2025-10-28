pub mod player;
pub mod prefix;
pub mod shop_item;
pub mod team;

use crate::collections::player::{MongoPlayerCollection, PlayerCollection};
use crate::collections::prefix::{MongoPrefixCollection, PrefixCollection};
use crate::collections::shop_item::{MongoShopItemCollection, ShopItemCollection};
use crate::collections::team::{MongoTeamCollection, TeamCollection};
use crate::models::player::Player;
use crate::models::prefix::Prefix;
use crate::models::shop_item::ShopItem;
use crate::models::team::Team;
use mongodb::Database;
use std::sync::Arc;

pub struct Collections {
    pub players: Arc<dyn PlayerCollection>,
    pub shop_items: Arc<dyn ShopItemCollection>,
    pub prefixes: Arc<dyn PrefixCollection>,
    pub teams: Arc<dyn TeamCollection>,
}

impl Collections {
    pub fn new(database: &Database) -> Self {
        let player_collection = MongoPlayerCollection {
            collection: database.collection::<Player>("players"),
        };

        let shop_item_collection = MongoShopItemCollection {
            collection: database.collection::<ShopItem>("shop_items"),
        };

        let prefix_collection = MongoPrefixCollection {
            collection: database.collection::<Prefix>("prefixes"),
        };

        let team_collection = MongoTeamCollection {
            collection: database.collection::<Team>("teams"),
        };

        Self {
            players: Arc::new(player_collection),
            shop_items: Arc::new(shop_item_collection),
            prefixes: Arc::new(prefix_collection),
            teams: Arc::new(team_collection),
        }
    }
}
