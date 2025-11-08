use tonic::{Request, Response, Status};
use tracing::warn;

use crate::{
    BridgeService,
    bridge::{SurvivalShutdownRequest, SurvivalShutdownResponse},
};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_survival_shutdown(
        &self,
        request: Request<SurvivalShutdownRequest>,
    ) -> Result<Response<SurvivalShutdownResponse>, Status> {
        let inner_request = request.into_inner();
        let client_id = inner_request.client_id;

        warn!("Survival shutdown requested by client {}", client_id);

        self.state().servers.survivals.remove(&client_id);

        warn!(
            "Survival shutdown request by client {} completed",
            client_id
        );

        Ok(Response::new(SurvivalShutdownResponse { success: true }))
    }
}
