use crate::BridgeService;
use crate::bridge::{GetFriendRequestsRequest, GetFriendRequestsResponse};
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

impl BridgeService {
    pub async fn handle_get_friend_requests(
        &self,
        request: Request<GetFriendRequestsRequest>,
    ) -> Result<Response<GetFriendRequestsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Get friend requests for player {} received", username);

        let player = self.cache.get_player_with_handling(&username).await?;

        let friends_requests = self
            .cache
            .get_friend_requests(&player)
            .await
            .map_err(|err| {
                error!(
                    "Failed to get the friend requests from index: {}",
                    err.to_string()
                );
                Status::internal(format!(
                    "Failed to get friend requests for player {}",
                    username
                ))
            })?;

        info!("Get friend requests for player {} completed", username);

        Ok(Response::new(GetFriendRequestsResponse {
            incoming_friend_requests: friends_requests.into_iter().collect(),
        }))
    }
}
