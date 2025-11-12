use crate::models::server::Server;
use crate::state::State;
use std::borrow::Borrow;

impl State {
    pub fn insert_server(&self, server: Server) {
        &self.servers.documents.insert(server.id, server.clone());
        &self
            .servers
            .addresses
            .insert(server.address.clone(), server.id.clone());
        &self
            .servers
            .names
            .insert(server.name.clone(), server.id.clone());

        if self.get_landing().is_none() {
            self.set_landing_server(Some(server.id));
        }
    }

    pub fn get_landing(&self) -> Option<u64> {
        *self.servers.landing.lock().unwrap()
    }

    pub fn set_landing_server(&self, id: Option<u64>) {
        let mut guard = self.servers.landing.lock().unwrap();
        *guard = id;
    }

    pub fn delete_server(&self, id: u64, address: String, name: String) {
        self.servers.documents.remove(&id);
        self.servers.addresses.remove(&address);
        self.servers.names.remove(&name);

        if self.get_landing().unwrap_or(0) == id {
            self.set_landing_server(None);
        }
    }
}
