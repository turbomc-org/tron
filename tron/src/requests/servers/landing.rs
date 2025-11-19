use crate::config::messages::SERVER_LANDING;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::error;
use tron_protos::{LandingServerRequest, LandingServerResponse};

impl BridgeService {
    pub async fn handle_landing_server(
        &self,
        request: Request<LandingServerRequest>,
    ) -> Result<Response<LandingServerResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let name = inner_request.name;

        let player = self.state().get_player_with_handling(&username).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can execute this command"),
                )
                .await;
        }

        let id = match self.state().servers.names.get(&name) {
            Some(r) => r.value().clone(),
            None => {
                return self
                    .status(
                        &username,
                        Status::not_found("Server you entered for landing doesn't exist."),
                    )
                    .await;
            }
        };

        let _ = {
            let mut landing = self.state().servers.landing.lock().unwrap();
            *landing = Some(id);
            id
        };

        if let Err(e) = self
            .send_message(&username, render!(SERVER_LANDING, name = name))
            .await
        {
            error!("Failed to send player message: {}", e);
        }

        Ok(Response::new(LandingServerResponse { success: true }))
    }
}
