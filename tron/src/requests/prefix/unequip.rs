use crate::BridgeService;
use crate::bridge::{UnEquipPrefixRequest, UnEquipPrefixResponse};
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_un_equip_prefix(
        &self,
        request: Request<UnEquipPrefixRequest>,
    ) -> Result<Response<UnEquipPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let mut player = self.state.get_player_with_handling(&username).await?;

        player
            .un_equip_prefix(&self.collections.players, &self.state)
            .await
            .map_err(|err| {
                error!("Failed to un-equip prefix: {}", err);
                Status::internal(format!("Failed to un-equip prefix: {}", err))
            })?;

        Ok(Response::new(UnEquipPrefixResponse { success: true }))
    }
}
