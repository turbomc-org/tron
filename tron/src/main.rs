use tron::grpc::GRPCService;
use tron::logger::Logger;

#[tokio::main]
async fn main() {
    Logger::init(false).await;
    GRPCService::init().await;
}
