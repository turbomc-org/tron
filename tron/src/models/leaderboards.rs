use dashmap::DashMap;
use indexset::BTreeMap;
use std::cmp::Reverse;
use tokio::sync::RwLock;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ScoreKey(pub Reverse<u64>, pub u64);

#[derive(Debug)]
pub struct Leaderboards {
    pub overall: Leaderboard,
    pub kills: Leaderboard,
    pub deaths: Leaderboard,
    pub teams: Leaderboard,
    pub coins: Leaderboard,
    pub kd: Leaderboard,
}

impl Leaderboards {
    pub fn new() -> Self {
        Leaderboards {
            overall: Leaderboard::new(),
            kills: Leaderboard::new(),
            deaths: Leaderboard::new(),
            teams: Leaderboard::new(),
            coins: Leaderboard::new(),
            kd: Leaderboard::new(),
        }
    }
}

#[derive(Debug)]
pub struct Leaderboard {
    pub order: RwLock<BTreeMap<ScoreKey, u64>>,
    pub index: DashMap<u64, ScoreKey>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Leaderboard {
            order: RwLock::new(BTreeMap::new()),
            index: DashMap::new(),
        }
    }
}
