use crate::{
    GENERATOR,
    bridge::{SurvivalStartupRequest, SurvivalStartupResponse},
};
use tonic::{Request, Response, Status};
use tracing::{info, warn};

use crate::BridgeService;

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_survival_startup(
        &self,
        request: Request<SurvivalStartupRequest>,
    ) -> Result<Response<SurvivalStartupResponse>, Status> {
        todo!("Implement survival startup handling")
    }
}
