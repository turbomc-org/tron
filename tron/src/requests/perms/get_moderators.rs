use crate::BridgeService;
use tonic::{Request, Response, Status};
use tracing::info;
use tron_protos::{GetAllModeratorsRequest, GetAllModeratorsResponse};

impl BridgeService {
    pub async fn handle_get_all_moderators(
        &self,
        request: Request<GetAllModeratorsRequest>,
    ) -> Result<Response<GetAllModeratorsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Get moderators request from player {} received", username);

        let player = self.player(&username).await?;

        if !player.is_admin() {
            return Err(Status::permission_denied("Your are not an admin."));
        }

        let moderator_ids: Vec<u64> = self
            .state()
            .permissions
            .moderators
            .iter()
            .map(|id| id.clone())
            .collect();
        let mut moderator_names: Vec<String> = Vec::new();

        for id in moderator_ids {
            if let Some(username) = self.state().get_player_username(&id) {
                moderator_names.push(username);
            } else {
                continue;
            }
        }

        info!("Get moderators request from player {} completed", username);

        Ok(Response::new(GetAllModeratorsResponse {
            usernames: moderator_names,
        }))
    }
}
