use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;
use tonic::Status;

pub struct Servers {
    pub proxies: Arc<DashMap<u64, bool>>,
    pub lobbies: Arc<DashMap<u64, bool>>,
    pub survivals: Arc<DashMap<u64, bool>>,
}

impl Servers {
    pub fn new() -> Self {
        Self {
            proxies: Arc::new(DashMap::new()),
            lobbies: Arc::new(DashMap::new()),
            survivals: Arc::new(DashMap::new()),
        }
    }
}

pub struct Client {}

impl Client {
    pub async fn get(cache: &Arc<DashMap<u64, bool>>, id: u64) -> Result<bool, Status> {
        let client = cache.get(&id).map(|c| c.clone());
        match client {
            Some(client) => Ok(client),
            None => Err(Status::not_found("Invalid client id")),
        }
    }
}
