use crate::GENERATOR;
use dashmap::{DashMap, DashSet};

#[derive(Debug)]
pub struct Messaging {
    pub streams: DashSet<u64>,
    pub subscriptions: DashMap<u64, u64>,
    pub team_streams: DashMap<u64, u64>,
    pub default_streams: DashMap<DefaultStreams, u64>,
    pub friend_chat_invites: DashMap<u64, u64>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum DefaultStreams {
    Global,
    Hindi,
}

impl Messaging {
    pub fn new() -> Self {
        let global_stream: u64 = GENERATOR.generate();
        let hindi_stream: u64 = GENERATOR.generate();
        let default_streams = DashMap::new();
        default_streams.insert(DefaultStreams::Global, global_stream);
        default_streams.insert(DefaultStreams::Hindi, hindi_stream);

        let streams = DashSet::new();
        streams.insert(global_stream);
        streams.insert(hindi_stream);

        Self {
            streams,
            subscriptions: DashMap::new(),
            team_streams: DashMap::new(),
            friend_chat_invites: DashMap::new(),
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

    pub fn exit_chat(&self, player_id: u64) {
        self.subscriptions.remove(&player_id);
    }

    pub fn create_stream(&self) -> u64 {
        let token = GENERATOR.generate();
        self.streams.insert(token);
        token
    }

    pub fn join_stream(&self, player_id: u64, stream_id: u64) {
        self.subscriptions.insert(player_id, stream_id);
    }

    pub fn join_global(&self, player_id: u64) {
        if let Some(global) = self.default_streams.get(&DefaultStreams::Global) {
            self.join_stream(player_id, *global);
        }
    }

    pub fn join_hindi(&self, player_id: u64) {
        if let Some(global) = self.default_streams.get(&DefaultStreams::Hindi) {
            self.join_stream(player_id, *global);
        }
    }

    pub fn is_in_global(&self, player_id: u64) -> bool {
        if let Some(global) = self.default_streams.get(&DefaultStreams::Global) {
            self.subscriptions.contains_key(&player_id)
                && *self.subscriptions.get(&player_id).unwrap() == *global
        } else {
            false
        }
    }

    pub fn is_in_hindi(&self, player_id: u64) -> bool {
        if let Some(hindi) = self.default_streams.get(&DefaultStreams::Hindi) {
            self.subscriptions.contains_key(&player_id)
                && *self.subscriptions.get(&player_id).unwrap() == *hindi
        } else {
            false
        }
    }

    pub fn join_team(&self, player_id: u64, team_id: u64) {
        if let Some(team) = self.team_streams.get(&team_id) {
            self.join_stream(player_id, *team);
        }
    }

    pub fn insert_request(&self, player_id: u64) -> u64 {
        let token = GENERATOR.generate();
        self.friend_chat_invites.insert(token, player_id);
        token
    }

    pub fn remove_request(&self, token: &u64) {
        self.friend_chat_invites.remove(token);
    }
}
