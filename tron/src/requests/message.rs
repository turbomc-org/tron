use crate::BridgeService;
use crate::MessageStream;
use crate::bridge::MessageRequest;
use async_stream::try_stream;
use futures::StreamExt;
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_message(
        &self,
        request: Request<tonic::Streaming<MessageRequest>>,
    ) -> Result<Response<MessageStream>, Status> {
        let mut inbound = request.into_inner();

        let stream = try_stream! {
            while let Some(req) = inbound.next().await {
                let req = req?;
                let filtered = Self::filter_message(&req.message);

                yield crate::bridge::MessageResponse {
                    username: "Harihar".to_string(),
                    message: filtered,
                    prefix: "123".to_string(),
                    badges: "548".to_string(),
                    team: "Decep".to_string(),
                    timestamp: 0
                };
            }
        };

        Ok(Response::new(Box::pin(stream) as MessageStream))
    }
}
