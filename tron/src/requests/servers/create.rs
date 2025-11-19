use crate::BridgeService;
use crate::models::player::Role;
use crate::models::server::Server;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{CreateServerRequest, CreateServerResponse};

impl BridgeService {
    pub async fn handle_create_server(
        &self,
        request: Request<CreateServerRequest>,
    ) -> Result<Response<CreateServerResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let name = inner_request.name;
        let description = inner_request.description;
        let address = inner_request.address;

        info!("Create server request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use create server command."),
                )
                .await;
        }

        if self.state().servers.addresses.contains_key(&address) {
            return self
                .status(
                    &username,
                    Status::already_exists("Server with this address is already registered."),
                )
                .await;
        }

        if self.state().servers.names.contains_key(&name) {
            return self
                .status(
                    &username,
                    Status::already_exists("Server with this name is already registered."),
                )
                .await;
        }

        let server = Server::new(player.id, name, description, address);

        if let Err(e) = server
            .insert(&self.collections().servers, &self.state())
            .await
        {
            return self
                .status(&username, Status::internal(e.to_string()))
                .await;
        }

        info!("Create server request from player {} completed", username);

        Ok(Response::new(CreateServerResponse { success: true }))
    }
}
