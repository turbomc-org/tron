use crate::GENERATOR;
use crate::collections::report::ReportCollection;
use crate::models::report::Report;
use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;

impl Report {
    pub fn new(player_id: u64, target_player_id: u64, reason: String) -> Self {
        let now: u64 = Utc::now().timestamp_millis() as u64;

        Report {
            id: GENERATOR.generate(),
            player_id,
            target_player_id,
            reason,
            created_at: now,
        }
    }

    pub async fn insert(&self, col: &Arc<dyn ReportCollection>) -> Result<()> {
        col.insert_one(&self).await?;

        Ok(())
    }

    pub async fn delete(&self, col: &Arc<dyn ReportCollection>) -> Result<()> {
        col.delete_one(self.id).await?;

        Ok(())
    }
}
