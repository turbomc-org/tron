use crate::BridgeService;
use crate::config::messages::REDEEM_DETAIL;
use crate::render;
use chrono::Utc;
use tonic::{Request, Response, Status};
use tron_protos::{ViewRedeemCodeRequest, ViewRedeemCodeResponse};

impl BridgeService {
    pub async fn handle_view_redeem_code(
        &self,
        request: Request<ViewRedeemCodeRequest>,
    ) -> Result<Response<ViewRedeemCodeResponse>, Status> {
        let request = request.into_inner();
        let username = request.username;
        let code = request.code;

        let player = self.player(&username).await?;

        if !player.is_admin() {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can view redeem codes."),
                )
                .await;
        }

        let id = match self.state().indexes.redeem.get(&code) {
            Some(id) => id.value().clone(),
            None => {
                return self
                    .status(&username, Status::not_found("Prefix not found."))
                    .await;
            }
        };

        let redeem = match self.state().redeems.get(&id) {
            Some(redeem) => redeem.value().clone(),
            None => {
                return self
                    .status(
                        &username,
                        Status::not_found("Redeem code record not found."),
                    )
                    .await;
            }
        };

        let now = Utc::now().timestamp() as u64;
        let elapsed_secs = now.saturating_sub(redeem.expires_at);
        let time = if elapsed_secs < 60 {
            "just now".to_string()
        } else if elapsed_secs < 3600 {
            format!("{} minute(s)", elapsed_secs / 60)
        } else if elapsed_secs < 86400 {
            format!("{} hour(s)", elapsed_secs / 3600)
        } else {
            format!("{} day(s)", elapsed_secs / 86400)
        };

        let reward = redeem.reward.as_string();

        self.send_message(
            &username,
            render!(
                REDEEM_DETAIL,
                id = &redeem.id,
                code = &redeem.code,
                reward = &reward,
                expiry = &time
            ),
        )
        .await;

        Ok(Response::new(ViewRedeemCodeResponse { success: true }))
    }
}
