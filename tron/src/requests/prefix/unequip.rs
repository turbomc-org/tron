use crate::bridge::{UnEquipPrefixRequest, UnEquipPrefixResponse};
use crate::config::messages::{IDENTIFIER_UNEQUIPPED, NO_ACTIVE_IDENTIFIER};
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::error;
use tracing::info;

impl BridgeService {
    pub async fn handle_un_equip_prefix(
        &self,
        request: Request<UnEquipPrefixRequest>,
    ) -> Result<Response<UnEquipPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("UnEquip prefix request from player {} received", username);

        let mut player = self.state().get_player_with_handling(&username).await?;

        if player.selected_prefix.is_none() {
            self.send_message(
                &username,
                render!(NO_ACTIVE_IDENTIFIER, username = &player.username),
            )
            .await
            .map_err(|err| {
                error!("Failed to send player message: {}", err);
            })
            .unwrap();

            return Err(Status::invalid_argument("Player has no prefix equipped"));
        }

        player
            .un_equip_prefix(&self.collections().players, &self.state())
            .await
            .map_err(|err| {
                error!("Failed to un-equip prefix: {}", err);
                Status::internal(format!("Failed to un-equip prefix: {}", err))
            })?;

        self.send_message(
            &username,
            render!(IDENTIFIER_UNEQUIPPED, username = &player.username),
        )
        .await
        .map_err(|err| {
            error!("Failed to send player message: {}", err);
        })
        .unwrap();

        info!("UnEquip prefix request from player {} completed", username);

        Ok(Response::new(UnEquipPrefixResponse { success: true }))
    }
}
