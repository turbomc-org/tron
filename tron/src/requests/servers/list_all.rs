use crate::BridgeService;
use crate::config::messages::{NO_SERVERS_FOUND, SERVER_DETAIL, SERVER_LIST};
use crate::models::player::Role;
use crate::models::server::Server;
use crate::render;
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{ListAllServersRequest, ListAllServersResponse};

impl BridgeService {
    pub async fn handle_list_all_servers(
        &self,
        request: Request<ListAllServersRequest>,
    ) -> Result<Response<ListAllServersResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("List all servers request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use list all server command."),
                )
                .await;
        }

        let servers: Vec<Server> = self
            .state()
            .servers
            .documents
            .iter()
            .map(|entry| entry.value().clone())
            .collect();

        if servers.is_empty() {
            self.send_message(&username, render!(NO_SERVERS_FOUND, username = &username))
                .await
                .map_err(|err| error!("Failed to send message: {}", err))
                .ok();
        } else {
            let now = Utc::now().timestamp() as u64;
            let mut entries = Vec::new();

            for server in &servers {
                let creator = match self.state().get_player_username(&server.created_by) {
                    Some(name) => name,
                    None => format!("Unknown ({})", server.created_by),
                };

                let elapsed_secs = now.saturating_sub(server.created_at);
                let time_ago = if elapsed_secs < 60 {
                    "just now".to_string()
                } else if elapsed_secs < 3600 {
                    format!("{} minute(s) ago", elapsed_secs / 60)
                } else if elapsed_secs < 86400 {
                    format!("{} hour(s) ago", elapsed_secs / 3600)
                } else {
                    format!("{} day(s) ago", elapsed_secs / 86400)
                };

                entries.push(render!(
                    SERVER_DETAIL,
                    id = server.id,
                    name = server.name,
                    description = server.description,
                    creator = creator,
                    created = time_ago,
                    address = server.address
                ));
            }

            let list_str = entries.join("\n\n");

            self.send_message(
                &username,
                render!(SERVER_LIST, count = &servers.len(), list = &list_str),
            )
            .await
            .map_err(|err| error!("Failed to send message: {}", err))
            .ok();
        }

        info!(
            "List all servers request from player {} completed",
            username
        );

        Ok(Response::new(ListAllServersResponse { success: true }))
    }
}
