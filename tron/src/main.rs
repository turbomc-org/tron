use tracing::info;
use tron::grpc::GRPCService;
use tron::logger::Logger;

#[tokio::main]
async fn main() {
    Logger::init(false).await;
    info!("initializing grpc service");
    GRPCService::init().await;
}
