use crate::RETRY_STRATEGY;
use crate::State;
use crate::bridge::Server as CompiledServer;
use crate::collections::server::ServerCollection;
use crate::models::server::Server;
use anyhow::Result;
use std::sync::Arc;
use tokio::task;
use tokio_retry::Retry;
use tracing::error;

impl Server {
    pub async fn insert(&self, col: &Arc<dyn ServerCollection>, state: &Arc<State>) -> Result<()> {
        let server = self.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.insert_one(&server).await.map_err(|e| {
                        error!("Retrying server insertion due to: {}", e);
                        e
                    })
                })
                .await;

                if let Err(e) = retry_result {
                    error!("Server insertion permanently failed: {}", e);
                }
            }
        });

        state.insert_server(self.clone());

        Ok(())
    }

    pub async fn delete(&self, col: &Arc<dyn ServerCollection>, state: &Arc<State>) -> Result<()> {
        let server_id = self.id.clone();

        task::spawn({
            let col = col.clone();
            async move {
                let retry_result = Retry::spawn(RETRY_STRATEGY.clone(), || async {
                    col.delete_one(server_id).await.map_err(|e| {
                        error!("Retrying server insertion due to: {}", e);
                        e
                    })
                })
                .await;

                if let Err(e) = retry_result {
                    error!("Server insertion permanently failed: {}", e);
                }
            }
        });

        state.delete_server(self.id, self.address.clone(), self.name.clone());

        Ok(())
    }

    pub fn compile(&self) -> CompiledServer {
        CompiledServer {
            id: self.id.clone(),
            address: self.address.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
        }
    }
}
