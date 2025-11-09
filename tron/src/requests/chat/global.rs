use crate::BridgeService;
use crate::bridge::{GlobalChatRequest, GlobalChatResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_global_chat(
        &self,
        request: Request<GlobalChatRequest>,
    ) -> Result<Response<GlobalChatResponse>, Status> {
        todo!("Implement global chat")
    }
}
