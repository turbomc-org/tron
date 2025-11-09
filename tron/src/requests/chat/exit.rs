use crate::BridgeService;
use crate::bridge::{ExitChatRequest, ExitChatResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_exit_chat(
        &self,
        request: Request<ExitChatRequest>,
    ) -> Result<Response<ExitChatResponse>, Status> {
        todo!("Implement exit chat")
    }
}
