use crate::BridgeService;
use crate::config::messages::{MODERATOR_LIST, MODERATOR_LIST_EMPTY};
use crate::render;
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{ListAllModeratorsRequest, ListAllModeratorsResponse};

impl BridgeService {
    pub async fn handle_list_all_moderators(
        &self,
        request: Request<ListAllModeratorsRequest>,
    ) -> Result<Response<ListAllModeratorsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("List moderators request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if !player.is_admin() {
            return Err(Status::permission_denied("You are not an admin"));
        }

        let moderator_ids: Vec<u64> = self
            .state()
            .permissions
            .moderators
            .iter()
            .map(|r| r.clone())
            .collect();
        let mut moderators: Vec<String> = Vec::new();

        for id in moderator_ids {
            if let Some(moderator) = self.state().get_player_username(&id) {
                moderators.push(moderator);
            } else {
                continue;
            }
        }

        if moderators.is_empty() {
            self.send_message(
                &username,
                render!(MODERATOR_LIST_EMPTY, username = username),
            )
            .await
            .map_err(|err| error!("Failed to send message: {}", err))
            .ok();
        } else {
            let mut entries = Vec::new();

            for username in &moderators {
                entries.push(format!(
                    "<dark_gray>»</dark_gray> <yellow><bold>@{}</bold></yellow> \
                     [<red><click:run_command:'/admin perms demote {}'>Delete</click></red>]",
                    username, username
                ));
            }

            let list_str = entries.join("\n\n");

            self.send_message(&username, render!(MODERATOR_LIST, list = &list_str))
                .await
                .map_err(|err| error!("Failed to send message: {}", err))
                .ok();
        }

        info!("List moderators request from player {} completed", username);

        Ok(Response::new(ListAllModeratorsResponse { success: true }))
    }
}
