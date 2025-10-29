use crate::BridgeService;
use crate::bridge::{DeletePrefixRequest, DeletePrefixResponse};
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_delete_prefix(
        &self,
        request: Request<DeletePrefixRequest>,
    ) -> Result<Response<DeletePrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let prefix_name = inner_request.prefix;

        let _ = self.state.get_player_with_handling(&username).await?;
        let prefix = self.state.get_prefix_with_handling(&prefix_name).await?;

        prefix
            .delete(&self.collections.prefixes, &self.state)
            .await
            .map_err(|err| {
                error!("Failed to delete prefix: {}", err);
                Status::internal("Failed to delete prefix")
            });

        Ok(Response::new(DeletePrefixResponse { success: true }))
    }
}
