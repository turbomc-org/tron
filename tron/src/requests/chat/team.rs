use crate::BridgeService;
use crate::bridge::{TeamChatRequest, TeamChatResponse};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_team_chat(
        &self,
        request: Request<TeamChatRequest>,
    ) -> Result<Response<TeamChatResponse>, Status> {
        todo!("Implement team chat")
    }
}
