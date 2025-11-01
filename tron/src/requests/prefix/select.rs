use crate::BridgeService;
use crate::bridge::{SelectPrefixRequest, SelectPrefixResponse};
use tonic::{Request, Response, Status};
use tracing::debug;
use tracing::error;

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_select_prefix(
        &self,
        request: Request<SelectPrefixRequest>,
    ) -> Result<Response<SelectPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let prefix_id = inner_request.prefix;

        debug!("Select prefix request from player {} received", username);

        let player = self.state.get_player_with_handling(&username).await?;
        let prefix = self.state.get_prefix_with_handling(&prefix_id).await?;

        if !player.prefixes.contains(&prefix.id) {
            error!("Player {} does not own prefix {}", username, prefix_id);
            return Err(Status::not_found("You don't own this prefix"));
        }

        unimplemented!()
    }
}
