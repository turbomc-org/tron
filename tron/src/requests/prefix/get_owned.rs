use crate::BridgeService;
use crate::bridge::{GetOwnedPrefixRequest, GetOwnedPrefixResponse};
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    #[cfg_attr(any(debug_assertions, test), tracing::instrument(skip(self), fields(request = ?request.get_ref())))]
    pub async fn handle_get_owned_prefixes(
        &self,
        request: Request<GetOwnedPrefixRequest>,
    ) -> Result<Response<GetOwnedPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!(
            "Get owned prefixes request from player {} received",
            username
        );

        let player = self.state().get_player_with_handling(&username).await?;

        let prefixes: Result<Vec<String>, Status> = player
            .prefixes
            .iter()
            .map(|prefix_id| {
                let state = self.state().clone();
                state
                    .get_prefix_text(prefix_id)
                    .ok_or_else(|| Status::not_found(format!("undefined prefix: {}", prefix_id)))
            })
            .collect();

        let prefixes = prefixes?;

        info!(
            "Get owned prefixes request from player {} completed",
            username
        );

        Ok(Response::new(GetOwnedPrefixResponse { prefixes }))
    }
}
