use dashmap::{DashMap, DashSet};

#[derive(Debug)]
pub struct Indexes {
    pub player: DashMap<u64, String>,
    pub team: DashMap<String, u64>,
    pub open_team: DashSet<u64>,
    pub prefix: DashMap<String, u64>,
    pub shop_item: DashMap<(String, Vec<String>), u64>,
    pub admin: DashSet<u64>,
    pub moderator: DashSet<u64>,
    pub redeem: DashMap<String, u64>,
}

impl Indexes {
    pub fn new() -> Self {
        Self {
            player: DashMap::new(),
            team: DashMap::new(),
            open_team: DashSet::new(),
            prefix: DashMap::new(),
            shop_item: DashMap::new(),
            admin: DashSet::new(),
            moderator: DashSet::new(),
            redeem: DashMap::new(),
        }
    }
}
