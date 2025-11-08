use crate::GENERATOR;
use dashmap::{DashMap, DashSet};

#[derive(Debug)]
pub struct Messaging {
    pub streams: DashSet<u64>,
    pub subscriptions: DashMap<u64, u64>,
    pub team_streams: DashMap<u64, u64>,
    pub default_streams: DashMap<DefaultStreams, u64>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum DefaultStreams {
    Global,
    Survival,
    Lobby,
}

impl Messaging {
    pub fn new() -> Self {
        let global_stream: u64 = GENERATOR.generate();
        let survival_stream: u64 = GENERATOR.generate();
        let lobby_stream: u64 = GENERATOR.generate();
        let default_streams = DashMap::new();
        default_streams.insert(DefaultStreams::Global, global_stream);
        default_streams.insert(DefaultStreams::Survival, survival_stream);
        default_streams.insert(DefaultStreams::Lobby, lobby_stream);

        Self {
            streams: DashSet::new(),
            subscriptions: DashMap::new(),
            team_streams: DashMap::new(),
            default_streams,
        }
    }

    pub fn register_team_stream(&self, team_id: u64) {
        let team_stream: u64 = GENERATOR.generate();
        self.streams.insert(team_stream);
        self.team_streams.insert(team_id, team_stream);
    }

    pub fn check_player(&self, player_id: u64) -> bool {
        self.subscriptions.contains_key(&player_id)
    }

    pub fn get_players_in_stream(&self, stream_id: u64) -> Vec<u64> {
        self.subscriptions
            .iter()
            .filter_map(|entry| {
                let (player_id, subscribed_stream) = entry.pair();
                if *subscribed_stream == stream_id {
                    Some(*player_id)
                } else {
                    None
                }
            })
            .collect()
    }
}
