use crate::BridgeService;
use crate::bridge::{GetAllBugsRequest, GetAllBugsResponse};
use crate::models::player::Role;
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    pub async fn handle_get_all_bugs(
        &self,
        request: Request<GetAllBugsRequest>,
    ) -> Result<Response<GetAllBugsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Get all bugs request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use get all bug command."),
                )
                .await;
        }

        let indexes = match self.collections().bugs.indexes().await {
            Ok(indexes) => indexes,
            Err(_) => {
                return self
                    .status(
                        &username,
                        Status::internal("Failed to fetch bug indexes from database."),
                    )
                    .await;
            }
        };

        let indexes = indexes.iter().map(|id| id.clone()).collect::<Vec<u64>>();

        info!("Get all bugs request from player {} completed", username);

        Ok(Response::new(GetAllBugsResponse { ids: indexes }))
    }
}
