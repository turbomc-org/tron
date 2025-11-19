use tron::grpc::GRPCService;
use tron::logger::Logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::init(false).await;
    GRPCService::init().await?;

    Ok(())
}
