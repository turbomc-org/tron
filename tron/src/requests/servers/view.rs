use crate::bridge::{ViewServerRequest, ViewServerResponse};
use crate::config::messages::SERVER_DETAIL;
use crate::utils::format_timestamp_indian_style;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_view_server(
        &self,
        request: Request<ViewServerRequest>,
    ) -> Result<Response<ViewServerResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let name = inner_request.name;

        info!("View server request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admin can view server details."),
                )
                .await;
        }

        if !self.state().servers.names.contains_key(&name) {
            return self
                .status(&username, Status::not_found("Server not found."))
                .await;
        }

        let id = match self.state().servers.names.get(&name) {
            Some(id) => id.value().clone(),
            None => {
                return self
                    .status(&username, Status::internal("Failed to retrieve server ID."))
                    .await;
            }
        };

        let server = match self.state().servers.documents.get(&id) {
            Some(server) => server.clone(),
            None => {
                return self
                    .status(
                        &username,
                        Status::internal("Failed to retrieve server details."),
                    )
                    .await;
            }
        };

        let created_by = match self.state().indexes.player.get(&server.created_by) {
            Some(r) => r.value().clone(),
            None => "Unknown".to_string(),
        };

        let created_at = format_timestamp_indian_style(server.created_at);

        if let Err(e) = self
            .send_message(
                &username,
                render!(
                    SERVER_DETAIL,
                    id = server.id,
                    name = server.name,
                    description = server.description,
                    created_by = created_by,
                    created = created_at,
                    address = server.address
                ),
            )
            .await
        {
            error!("Failed to send player message: {}", e);
        };

        info!("View server request from player {} completed", username);

        Ok(Response::new(ViewServerResponse { success: true }))
    }
}
