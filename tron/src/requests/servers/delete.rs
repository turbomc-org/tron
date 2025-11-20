use crate::BridgeService;
use crate::models::player::Role;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{DeleteServerRequest, DeleteServerResponse};

impl BridgeService {
    pub async fn handle_delete_server(
        &self,
        request: Request<DeleteServerRequest>,
    ) -> Result<Response<DeleteServerResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let server_name = inner_request.name;

        info!("Delete server request from player {} received", username);

        let player = self.player(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use delete server command."),
                )
                .await;
        }

        if !self.state().servers.names.contains_key(&server_name) {
            return self
                .status(
                    &username,
                    Status::not_found(format!("Server with name {} not found.", server_name)),
                )
                .await;
        }

        let server_id = match self.state().servers.names.get(&server_name) {
            Some(ref_multi) => *ref_multi.value(),
            None => {
                return self
                    .status(
                        &username,
                        Status::not_found(format!("Server with name {} not found.", server_name)),
                    )
                    .await;
            }
        };

        let server = match self.state().servers.documents.get(&server_id) {
            Some(ref_multi) => ref_multi.value().clone(),
            None => {
                return self
                    .status(
                        &username,
                        Status::not_found(format!("Server with id {} not found.", server_id)),
                    )
                    .await;
            }
        };

        if let Err(e) = server
            .delete(&self.collections().servers, &self.state())
            .await
        {
            error!("Failed to delete server: {}", e);
            return self
                .status(
                    &username,
                    Status::internal(format!("Failed to delete server")),
                )
                .await;
        }

        info!("Delete server request from player {} completed", username);

        Ok(Response::new(DeleteServerResponse { success: true }))
    }
}
