use crate::collections::bug::BugCollection;
use crate::models::bug::Bug;
use anyhow::Result;
use std::sync::Arc;

impl Bug {
    pub async fn insert(&self, col: &Arc<dyn BugCollection>) -> Result<()> {
        col.insert_one(&self).await?;

        Ok(())
    }

    pub async fn delete(&self, col: &Arc<dyn BugCollection>) -> Result<()> {
        col.delete_one(self.id).await?;

        Ok(())
    }
}
