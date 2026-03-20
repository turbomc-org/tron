use crate::BridgeService;
use crate::collections::Collections;
use crate::utils::mongodb::MongoDB;
use tonic::transport::Server;
use tracing::info;
use tron_protos::bridge_server::BridgeServer;

pub struct GRPCService {}

impl GRPCService {
    pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let addr = std::env::var("TRON_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:50051".to_string())
            .parse()
            .expect("failed to parse the address");
        info!("🟩 Listener running on {}", addr);

        let mongodb = MongoDB::new()
            .await
            .expect("Failed to establish MongoDB connection");

        let database = mongodb.database;
        let collections = Collections::new(&database);

        let bs = BridgeService::new(collections).await;
        let svc = BridgeServer::new(bs);

        Server::builder().add_service(svc).serve(addr).await?;

        Ok(())
    }
}
