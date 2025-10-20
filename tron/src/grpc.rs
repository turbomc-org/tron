use crate::BridgeService;
use crate::bridge::bridge_server::BridgeServer;
use tonic::transport::Server;
use tracing::info;

pub struct GRPCService {}

impl GRPCService {
    pub async fn init() {
        let addr = "127.0.0.1:50051"
            .parse()
            .expect("failed to parse the address");
        info!("🟩 Listener running on {}", addr);

        let svc = BridgeServer::new(BridgeService::default());
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
            .expect("failed to start the server");
    }
}
