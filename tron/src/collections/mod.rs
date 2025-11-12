pub mod bug;
pub mod player;
pub mod prefix;
pub mod report;
pub mod server;
pub mod shop_item;
pub mod team;

use crate::collections::bug::{BugCollection, MongoBugCollection};
use crate::collections::player::{MongoPlayerCollection, PlayerCollection};
use crate::collections::prefix::{MongoPrefixCollection, PrefixCollection};
use crate::collections::report::{MongoReportCollection, ReportCollection};
use crate::collections::server::{MongoServerCollection, ServerCollection};
use crate::collections::shop_item::{MongoShopItemCollection, ShopItemCollection};
use crate::collections::team::{MongoTeamCollection, TeamCollection};
use crate::models::bug::Bug;
use crate::models::player::Player;
use crate::models::prefix::Prefix;
use crate::models::report::Report;
use crate::models::server::Server;
use crate::models::shop_item::ShopItem;
use crate::models::team::Team;
use mongodb::Database;
use std::sync::Arc;

#[derive(Debug)]
pub struct Collections {
    pub players: Arc<dyn PlayerCollection>,
    pub shop_items: Arc<dyn ShopItemCollection>,
    pub prefixes: Arc<dyn PrefixCollection>,
    pub teams: Arc<dyn TeamCollection>,
    pub reports: Arc<dyn ReportCollection>,
    pub bugs: Arc<dyn BugCollection>,
    pub servers: Arc<dyn ServerCollection>,
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

        let report_collection = MongoReportCollection {
            collection: database.collection::<Report>("reports"),
        };

        let bug_collections = MongoBugCollection {
            collection: database.collection::<Bug>("bugs"),
        };

        let server_collection = MongoServerCollection {
            collection: database.collection::<Server>("servers"),
        };

        Self {
            players: Arc::new(player_collection),
            shop_items: Arc::new(shop_item_collection),
            prefixes: Arc::new(prefix_collection),
            teams: Arc::new(team_collection),
            reports: Arc::new(report_collection),
            bugs: Arc::new(bug_collections),
            servers: Arc::new(server_collection),
        }
    }
}
