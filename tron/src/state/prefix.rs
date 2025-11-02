use crate::models::prefix::Prefix;
use crate::state::State;
use anyhow::Result;
use anyhow::anyhow;
use tonic::Status;

impl State {
    pub async fn get_prefixes(&self) -> Result<Vec<Prefix>> {
        Ok(self.prefixes.iter().map(|p| p.clone()).collect())
    }

    pub fn get_prefix_text(&self, id: &u64) -> Option<String> {
        match self.prefixes.get(id).map(|entry| entry.text.clone()) {
            Some(text) => Some(text),
            None => None,
        }
    }

    pub async fn get_prefix(&self, id: &u64) -> Result<Prefix> {
        match self.prefixes.get(id).map(|entry| entry.clone()) {
            Some(prefix) => Ok(prefix),
            None => Err(anyhow!("Requested prefix not found in database")),
        }
    }

    pub async fn insert_prefix(&self, prefix: Prefix) -> Result<()> {
        self.prefixes.insert(prefix.id.clone(), prefix.clone());
        self.prefix_indexes
            .insert(prefix.text.clone(), prefix.id.clone());
        Ok(())
    }

    pub async fn remove_prefix(&self, id: &u64, text: &String) -> Result<()> {
        self.prefixes.remove(id);
        self.prefix_indexes.remove(text);
        Ok(())
    }

    pub async fn get_prefix_with_handling(&self, name: &String) -> Result<Prefix, Status> {
        let id = self
            .prefix_indexes
            .get(name)
            .ok_or_else(|| Status::not_found("Requested prefix not found in database"))?;

        match self.prefixes.get(&id).map(|entry| entry.clone()) {
            Some(team) => Ok(team),
            None => Err(Status::not_found("Requested prefix not found in database")),
        }
    }
}
