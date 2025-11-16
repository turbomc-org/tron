use crate::bridge::{ListAllAdminsRequest, ListAllAdminsResponse};
use crate::config::messages::{ADMIN_LIST, ADMIN_LIST_EMPTY};
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_list_all_admins(
        &self,
        request: Request<ListAllAdminsRequest>,
    ) -> Result<Response<ListAllAdminsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("List admins request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use list admins command."),
                )
                .await;
        }

        let admin_ids: Vec<u64> = self
            .state()
            .permissions
            .admins
            .iter()
            .map(|r| r.clone())
            .collect();
        let mut admins: Vec<String> = Vec::new();

        for id in admin_ids {
            if let Some(admin) = self.state().get_player_username(&id) {
                admins.push(admin);
            } else {
                continue;
            }
        }

        if admins.is_empty() {
            self.send_message(&username, render!(ADMIN_LIST_EMPTY, username = username))
                .await
                .map_err(|err| error!("Failed to send message: {}", err))
                .ok();
        } else {
            let mut entries = Vec::new();

            for username in &admins {
                entries.push(format!(
                    "<dark_gray>»</dark_gray> <yellow><bold>@{}</bold></yellow> \
                     [<red><click:run_command:'/admin perms demote {}'>Delete</click></red>]",
                    username, username
                ));
            }

            let list_str = entries.join("\n\n");

            self.send_message(&username, render!(ADMIN_LIST, list = &list_str))
                .await
                .map_err(|err| error!("Failed to send message: {}", err))
                .ok();
        }

        info!("List admins request from player {} completed", username);

        Ok(Response::new(ListAllAdminsResponse { success: true }))
    }
}
