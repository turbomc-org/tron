use crate::BridgeService;
use crate::bridge::{SendMessageRequest, SendMessageResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        unimplemented!()
    }
}
