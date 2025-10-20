use crate::models::player::Player;
use crate::models::prefix::Prefix;
use crate::models::shop_item::ShopItem;
use crate::models::team::Team;
use mongodb::Collection;
use mongodb::Database;

pub struct Databases {
    pub players: Collection<Player>,
    pub shop_items: Collection<ShopItem>,
    pub prefixes: Collection<Prefix>,
    pub teams: Collection<Team>,
}

impl Databases {
    pub fn new(database: &Database) -> Self {
        Self {
            players: database.collection("players"),
            shop_items: database.collection("shop_items"),
            prefixes: database.collection("prefixes"),
            teams: database.collection("teams"),
        }
    }
}
