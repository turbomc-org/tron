use dashmap::DashMap;

use crate::models::server::Server;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Servers {
    pub addresses: DashMap<String, u64>,
    pub names: DashMap<String, u64>,
    pub documents: DashMap<u64, Server>,
    pub landing: Arc<Mutex<Option<u64>>>,
}

impl Servers {
    pub fn new() -> Self {
        Self {
            addresses: DashMap::new(),
            names: DashMap::new(),
            documents: DashMap::new(),
            landing: Arc::new(Mutex::new(None)),
        }
    }
}
