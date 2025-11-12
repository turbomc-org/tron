use crate::BridgeService;
use crate::bridge::{FriendChatRequest, FriendChatResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_friend_chat(
        &self,
        request: Request<FriendChatRequest>,
    ) -> Result<Response<FriendChatResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let friend_username = inner_request.friend;

        let player = self.state().get_player_with_handling(&username).await?;
        let friend = self
            .state()
            .get_player_with_handling(&friend_username)
            .await?;

        todo!("Implement friend chat")
    }
}
