use anyhow::Result;
use dashmap::DashMap;
use dashmap::DashSet;
use std::sync::Arc;
use tonic::Status;

#[derive(Debug)]
pub struct Servers {
    pub proxies: DashSet<u64>,
    pub lobbies: DashSet<u64>,
    pub survivals: DashSet<u64>,
}

impl Servers {
    pub fn new() -> Self {
        Self {
            proxies: DashSet::new(),
            lobbies: DashSet::new(),
            survivals: DashSet::new(),
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
