use crate::bridge::{EquipPrefixRequest, EquipPrefixResponse};
use crate::config::messages::IDENTIFIER_EQUIPPED;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::error;
use tracing::info;

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_equip_prefix(
        &self,
        request: Request<EquipPrefixRequest>,
    ) -> Result<Response<EquipPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let prefix_id = inner_request.prefix;

        info!("Equip prefix request from player {} received", username);

        let mut player = self.state().get_player_with_handling(&username).await?;
        let prefix = self.state().get_prefix_with_handling(&prefix_id).await?;

        if !player.prefixes.contains(&prefix.id) {
            error!("Player {} does not own prefix {}", username, prefix_id);
            return Err(Status::not_found("You don't own this prefix"));
        }

        prefix
            .select(&mut player, &self.collections().players, &self.state())
            .await
            .map_err(|err| {
                error!("Failed to select prefix: {}", err);
                Status::internal("Failed to equip prefix")
            })?;

        self.send_message(
            &username,
            render!(
                IDENTIFIER_EQUIPPED,
                color = &prefix.color,
                text = &prefix.text
            ),
        )
        .await;

        info!("Equip prefix request from player {} completed", username);

        Ok(Response::new(EquipPrefixResponse { success: true }))
    }
}
