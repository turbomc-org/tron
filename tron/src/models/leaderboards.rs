use dashmap::DashMap;
use std::sync::Arc;

pub struct Leaderboards {
    pub overall: Arc<DashMap<u64, u64>>,
    pub kills: Arc<DashMap<u64, u64>>,
    pub deaths: Arc<DashMap<u64, u64>>,
    pub assists: Arc<DashMap<u64, u64>>,
    pub teams: Arc<DashMap<u64, u64>>,
    pub coins: Arc<DashMap<u64, u64>>,
    pub kda: Arc<DashMap<u64, f64>>,
}

impl Leaderboards {
    pub fn new() -> Self {
        Leaderboards {
            overall: Arc::new(DashMap::new()),
            kills: Arc::new(DashMap::new()),
            deaths: Arc::new(DashMap::new()),
            assists: Arc::new(DashMap::new()),
            teams: Arc::new(DashMap::new()),
            coins: Arc::new(DashMap::new()),
            kda: Arc::new(DashMap::new()),
        }
    }
}
