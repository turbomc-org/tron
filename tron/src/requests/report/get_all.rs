use crate::BridgeService;
use crate::models::player::Role;
use tonic::{Request, Response, Status};
use tron_protos::{GetAllReportsRequest, GetAllReportsResponse};

impl BridgeService {
    pub async fn handle_get_all_reports(
        &self,
        request: Request<GetAllReportsRequest>,
    ) -> Result<Response<GetAllReportsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        let player = self.state().get_player_with_handling(&username).await?;

        if player.role != Role::Admin {
            return self
                .status(
                    &username,
                    Status::permission_denied("Only admins can use get all report command."),
                )
                .await;
        }

        let indexes = match self.collections().reports.indexes().await {
            Ok(indexes) => indexes,
            Err(_) => {
                return self
                    .status(
                        &username,
                        Status::internal("Failed to fetch report indexes from database."),
                    )
                    .await;
            }
        };

        let indexes = indexes.iter().map(|id| id.clone()).collect::<Vec<u64>>();

        Ok(Response::new(GetAllReportsResponse { ids: indexes }))
    }
}
