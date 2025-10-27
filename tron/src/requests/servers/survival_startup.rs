use crate::{
    GENERATOR,
    bridge::{SurvivalStartupRequest, SurvivalStartupResponse},
};
use tonic::{Request, Response, Status};
use tracing::{info, warn};

use crate::BridgeService;

impl BridgeService {
    pub async fn handle_survival_startup(
        &self,
        _request: Request<SurvivalStartupRequest>,
    ) -> Result<Response<SurvivalStartupResponse>, Status> {
        warn!("Survival startup request received");

        let client_id = GENERATOR.generate();
        self.cache.servers.survivals.insert(client_id, true);

        info!("New survival client registered {}", client_id);
        warn!("Survival startup request completed");

        Ok(Response::new(SurvivalStartupResponse { client_id }))
    }
}

#[cfg(test)]
mod tests {
    use crate::BridgeService;
    use crate::bridge::SurvivalStartupRequest;
    use crate::logger::Logger;
    use tonic::Request;

    #[tokio::test]
    async fn test_handle_survival_startup() {
        Logger::init(true).await;
        let service = BridgeService::new().await;

        let req = Request::new(SurvivalStartupRequest {});

        let resp = service
            .handle_survival_startup(req)
            .await
            .unwrap()
            .into_inner();

        let client_id = resp.client_id;

        assert!(service.cache.servers.survivals.contains_key(&client_id));
    }
}
