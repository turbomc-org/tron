use crate::BridgeService;
use crate::bridge::{GetFriendsRequest, GetFriendsResponse};
use futures::future::join_all;
use tonic::{Request, Response, Status};
use tracing::debug;

impl BridgeService {
    #[tracing::instrument]
    pub async fn handle_get_friends(
        &self,
        request: Request<GetFriendsRequest>,
    ) -> Result<Response<GetFriendsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Get friends request for player {} received", username);

        let player = self.state.get_player_with_handling(&username).await?;

        let friend_futures = player.friends.iter().map(|friend_id| {
            let state = self.state.clone();
            async move {
                state
                    .get_player_username(friend_id)
                    .await
                    .map_err(|_| Status::internal("Failed to fetch friend username"))
            }
        });

        let results = join_all(friend_futures).await;

        let mut friends = Vec::new();
        for res in results {
            match res {
                Ok(Some(username)) => friends.push(username),
                Ok(None) => continue,
                Err(status) => return Err(status),
            }
        }

        debug!("Get friends request for player {} completed", username);

        Ok(Response::new(GetFriendsResponse { friends }))
    }
}
