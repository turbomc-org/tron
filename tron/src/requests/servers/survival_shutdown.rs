use tonic::{Request, Response, Status};
use tracing::warn;

use crate::{
    BridgeService,
    bridge::{SurvivalShutdownRequest, SurvivalShutdownResponse},
    models::servers::Client,
};

impl BridgeService {
    pub async fn handle_survival_shutdown(
        &self,
        request: Request<SurvivalShutdownRequest>,
    ) -> Result<Response<SurvivalShutdownResponse>, Status> {
        let inner_request = request.into_inner();
        let client_id = inner_request.client_id;

        warn!("Survival shutdown requested by client {}", client_id);

        let client = Client::get(&self.cache.servers.survivals, client_id).await?;

        if !client {
            return Err(Status::not_found("Survival server is not active"));
        }

        self.cache.servers.survivals.remove(&client_id);

        warn!(
            "Survival shutdown request by client {} completed",
            client_id
        );

        Ok(Response::new(SurvivalShutdownResponse { success: true }))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::BridgeService;
//     use crate::bridge::{SurvivalShutdownRequest, SurvivalStartupRequest};
//     use crate::logger::Logger;
//     use tonic::Request;

//     #[tokio::test]
//     async fn test_handle_survival_shutdown() {
//         Logger::init(true).await;
//         let service = BridgeService::new().await;

//         let s_req = Request::new(SurvivalStartupRequest {});

//         let s_resp = service
//             .handle_survival_startup(s_req)
//             .await
//             .unwrap()
//             .into_inner();

//         let client_id = s_resp.client_id;

//         assert!(service.cache.servers.survivals.contains_key(&client_id));

//         let sh_req = Request::new(SurvivalShutdownRequest { client_id });

//         let s_resp = service
//             .handle_survival_shutdown(sh_req)
//             .await
//             .unwrap()
//             .into_inner();

//         assert!(s_resp.success);
//         assert!(!service.cache.servers.survivals.contains_key(&client_id));
//     }
// }
