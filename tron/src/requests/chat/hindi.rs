use crate::BridgeService;
use crate::bridge::{HindiChatRequest, HindiChatResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_hindi_chat(
        &self,
        request: Request<HindiChatRequest>,
    ) -> Result<Response<HindiChatResponse>, Status> {
        todo!("Implement global chat")
    }
}
