use crate::BridgeService;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{GetFriendsRequest, GetFriendsResponse};

impl BridgeService {
    pub async fn handle_get_friends(
        &self,
        request: Request<GetFriendsRequest>,
    ) -> Result<Response<GetFriendsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username.clone();

        info!("Get friends request for player {} received", username);

        let player = self.player(&username).await?;

        let mut friends: Vec<String> = Vec::new();
        for friend_id in &player.friends {
            if let Some(friend_name) = self.state().get_player_username(friend_id) {
                friends.push(friend_name);
            }
        }

        info!("Get friends request for player {} completed", username);

        Ok(Response::new(GetFriendsResponse { friends }))
    }
}
