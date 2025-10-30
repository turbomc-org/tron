use crate::BridgeService;
use crate::bridge::{GetFriendRequestsRequest, GetFriendRequestsResponse};
use futures::future::join_all;
use std::collections::HashMap;
use tonic::{Request, Response, Status};
use tracing::debug;

impl BridgeService {
    #[tracing::instrument]
    pub async fn handle_get_friend_requests(
        &self,
        request: Request<GetFriendRequestsRequest>,
    ) -> Result<Response<GetFriendRequestsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        debug!("Get friend requests for player {} received", username);

        let player = self.state.get_player_with_handling(&username).await?;

        let incoming_friend_futures =
            player
                .incoming_friend_requests
                .iter()
                .map(|(sender_id, now)| {
                    let state = self.state.clone();
                    async move {
                        match state
                            .get_player_username(sender_id)
                            .await
                            .map_err(|_| Status::internal("Failed to fetch friend username"))?
                        {
                            Some(username) => Ok::<(String, u64), Status>((username, *now)),
                            None => Err(Status::not_found("Friend record not found")),
                        }
                    }
                });

        let results = join_all(incoming_friend_futures).await;

        let mut incoming_friend_requests = HashMap::new();
        for res in results {
            match res {
                Ok((username, now)) => {
                    incoming_friend_requests.insert(username, now);
                }
                Err(status) => {
                    return Err(status);
                }
            }
        }

        debug!("Get friend requests for player {} completed", username);

        Ok(Response::new(GetFriendRequestsResponse {
            incoming_friend_requests,
        }))
    }
}
