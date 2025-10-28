use crate::RETRY_STRATEGY;
use crate::collections::prefix::PrefixCollection;
use crate::models::prefix::Prefix;
use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::task;
use tokio_retry::Retry;
use tonic::Status;
use tracing::error;

impl Prefix {
    pub async fn insert(&self, col: &Arc<dyn PrefixCollection>, cache: &Arc<DashMap<u64, Prefix>>) {
        let prefix = self.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.insert_one(&prefix).await.map_err(|e| {
                        error!("Retrying prefix insertion due to: {}", e);
                        e
                    })
                })
                .await;

                if let Err(e) = retry_result {
                    error!("Prefix insertion permanently failed: {}", e);
                }
            }
        });

        cache.insert(self.id, self.clone());
    }

    pub async fn delete(
        &self,
        col: &Arc<dyn PrefixCollection>,
        cache: &Arc<DashMap<u64, Prefix>>,
    ) -> Result<(), Status> {
        let prefix_id = self.id.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.delete_one(prefix_id).await.map_err(|e| {
                        error!("Retrying prefix insertion due to: {}", e);
                        e
                    })
                })
                .await;

                if let Err(e) = retry_result {
                    error!("Prefix insertion permanently failed: {}", e);
                }
            }
        });

        cache.remove(&self.id);

        Ok(())
    }
}
