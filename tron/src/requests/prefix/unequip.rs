use crate::BridgeService;
use crate::bridge::{UnEquipPrefixRequest, UnEquipPrefixResponse};
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

        let mut player = self.state.get_player_with_handling(&username).await?;

        if player.selected_prefix.is_none() {
            self.send_message_to_player(
              &username,
              format!(
                "<gradient:#C724B1:#7A00FF><bold>ℹ️ NO ACTIVE IDENTIFIER</bold></gradient>\n\
                 <gray>You do not have a network identifier equipped to unequip.</gray>\n\
                 <dark_gray>»</dark_gray> <click:run_command:'/prefixes'><u><gradient:#B200FF:#6A00A3>Select an identifier to equip</gradient></u></click>"
              ),
            ).await;

            return Err(Status::invalid_argument("Player has no prefix equipped"));
        }

        player
            .un_equip_prefix(&self.collections.players, &self.state)
            .await
            .map_err(|err| {
                error!("Failed to un-equip prefix: {}", err);
                Status::internal(format!("Failed to un-equip prefix: {}", err))
            })?;

        self.send_message_to_player(
          &username,
          format!(
            "<gradient:#C724B1:#7A00FF><bold>✅ IDENTIFIER UNEQUIPPED</bold></gradient>\n\
             <gray>Your network identifier has been reset to default.</gray>\n\
             <dark_gray>»</dark_gray> <click:run_command:'/prefix owned'><u><gradient:#B200FF:#6A00A3>Browse your collection</gradient></u></click>"
          ),
        ).await;

        info!("UnEquip prefix request from player {} completed", username);

        Ok(Response::new(UnEquipPrefixResponse { success: true }))
    }
}
