use tron_protos::{DeletePrefixRequest, DeletePrefixResponse};
use crate::config::messages::ASSET_PURGED;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_delete_prefix(
        &self,
        request: Request<DeletePrefixRequest>,
    ) -> Result<Response<DeletePrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let prefix_name = inner_request.prefix;

        info!("Delete request from player {} received", username);

        let _ = self.state().get_player_with_handling(&username).await?;
        let prefix = self.state().get_prefix_with_handling(&prefix_name).await?;

        prefix
            .delete(&self.collections().prefixes, &self.state())
            .await
            .map_err(|err| {
                error!("Failed to delete prefix: {}", err);
                Status::internal("Failed to delete prefix")
            })?;

        self.send_message(&username, render!(ASSET_PURGED, name = &prefix_name))
            .await
            .map_err(|err| {
                error!("Failed to send player message: {}", err);
            })
            .unwrap();

        info!("Delete request from player {} completed", username);

        Ok(Response::new(DeletePrefixResponse { success: true }))
    }
}
