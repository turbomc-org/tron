use crate::BridgeService;
use crate::bridge::{FriendChatRequest, FriendChatResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_friend_chat(
        &self,
        request: Request<FriendChatRequest>,
    ) -> Result<Response<FriendChatResponse>, Status> {
        todo!("Implement friend chat")
    }
}
