use crate::BridgeService;
use crate::bridge::{GetFriendsRequest, GetFriendsResponse};
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    pub async fn handle_get_friends(
        &self,
        request: Request<GetFriendsRequest>,
    ) -> Result<Response<GetFriendsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Get friends request for player {} received", username);

        let player = self.cache.get_player_with_handling(&username).await?;

        let friends = self
            .cache
            .get_friend_usernames(&player)
            .await
            .map_err(|err| {
                error!(
                    "Failed to fetch friend usernames for player {}: {}",
                    username, err
                );
                Status::internal(format!(
                    "Failed to fetch friend usernames for player {}",
                    username
                ))
            })?;

        info!(
            "Get friends request received for player {} completed",
            username
        );

        Ok(Response::new(GetFriendsResponse { friends }))
    }
}
