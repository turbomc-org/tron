use tonic::{Request, Response, Status};
use tracing::warn;

use crate::{
    BridgeService,
    bridge::{SurvivalShutdownRequest, SurvivalShutdownResponse},
};

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_survival_shutdown(
        &self,
        request: Request<SurvivalShutdownRequest>,
    ) -> Result<Response<SurvivalShutdownResponse>, Status> {
        todo!("Implement survival shutdown handling")
    }
}
