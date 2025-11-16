use crate::BridgeService;
use crate::bridge::{ListAllRedeemCodesRequest, ListAllRedeemCodesResponse};
use crate::config::messages::{EMPTY_REDEEM_CODES, REDEEM_LIST};
use crate::models::redeem::Redeem;
use crate::render;
use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    pub async fn handle_list_all_redeem_codes(
        &self,
        request: Request<ListAllRedeemCodesRequest>,
    ) -> Result<Response<ListAllRedeemCodesResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!(
            "List all redeem codes request from player {} received",
            username
        );

        let player = self.state().get_player_with_handling(&username).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can list all the redeem codes."),
                )
                .await;
        }

        let redeems: Vec<Redeem> = self
            .state()
            .redeems
            .iter()
            .map(|r| r.value().clone())
            .collect();

        if redeems.is_empty() {
            self.send_message(&username, render!(EMPTY_REDEEM_CODES, username = &username))
                .await
                .map_err(|err| error!("Failed to send message: {}", err))
                .ok();
        } else {
            let now = Utc::now().timestamp() as u64;
            let mut entries = Vec::new();

            for redeem in &redeems {
                let created_by = match self.state().get_player_username(&redeem.created_by) {
                    Some(name) => name,
                    None => format!("Unknown ({})", redeem.created_by),
                };

                let elapsed_secs = now.saturating_sub(redeem.expires_at);
                let time_ago = if elapsed_secs < 60 {
                    "just now".to_string()
                } else if elapsed_secs < 3600 {
                    format!("{} minute(s) ago", elapsed_secs / 60)
                } else if elapsed_secs < 86400 {
                    format!("{} hour(s) ago", elapsed_secs / 3600)
                } else {
                    format!("{} day(s) ago", elapsed_secs / 86400)
                };

                entries.push(format!(
                    "<dark_gray>»</dark_gray> <yellow><bold>#{}</bold></yellow> \
                     <gray>-</gray> <light_purple>{}</light_purple> \
                     <gray>by</gray> <aqua>{}</aqua> <gray>Expiry: ({})</gray>\n\
                     <dark_gray>   └─</dark_gray> [<green><click:run_command:'/admin redeem view {}'>View</click></green>] \
                     [<red><click:run_command:'/admin redeem delete {}'>Delete</click></red>]",
                    redeem.id, redeem.code, created_by, time_ago, redeem.id, redeem.id
                ));
            }

            let list_str = entries.join("\n\n");

            self.send_message(&username, render!(REDEEM_LIST, list = &list_str))
                .await
                .map_err(|err| error!("Failed to send message: {}", err))
                .ok();
        }

        info!(
            "List all redeem codes request from player {} completed",
            username
        );

        Ok(Response::new(ListAllRedeemCodesResponse { success: true }))
    }
}
