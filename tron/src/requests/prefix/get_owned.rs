use crate::BridgeService;
use crate::bridge::{GetOwnedPrefixRequest, GetOwnedPrefixResponse};
use futures::future::join_all;
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
    pub async fn handle_get_owned_prefix(
        &self,
        request: Request<GetOwnedPrefixRequest>,
    ) -> Result<Response<GetOwnedPrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!(
            "Get owned prefixes request from player {} received",
            username
        );

        let player = self.state.get_player_with_handling(&username).await?;

        let prefixes_future = player.prefixes.iter().map(|prefix_id| {
            let state = self.state.clone();
            async move {
                let text = state
                    .get_prefix_text(prefix_id)
                    .await
                    .map_err(|_| Status::internal("Failed to fetch prefix text"))?;

                Ok(text)
            }
        });

        let results = join_all(prefixes_future).await;

        let mut prefixes = Vec::new();
        for res in results {
            match res {
                Ok(prefix) => prefixes.push(prefix),
                Err(status) => return Err(status),
            }
        }

        info!(
            "Get owned prefixes request from player {} completed",
            username
        );

        Ok(Response::new(GetOwnedPrefixResponse { prefixes }))
    }
}
