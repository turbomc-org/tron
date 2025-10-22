use crate::cache::Cache;
use crate::models::prefix::Prefix;
use anyhow::Result;

impl Cache {
    pub async fn get_prefix(&self, id: &u64) -> Result<Option<Prefix>> {
        Ok(self.prefixes.get(id).map(|entry| entry.clone()))
    }

    pub async fn insert_prefix(&self, prefix: Prefix) -> Result<()> {
        self.prefixes.insert(prefix.id, prefix);
        Ok(())
    }
}
