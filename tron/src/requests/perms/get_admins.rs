use crate::BridgeService;
use crate::bridge::{GetAllAdminsRequest, GetAllAdminsResponse};
use tonic::{Request, Response, Status};
use tracing::info;

impl BridgeService {
    pub async fn handle_get_all_admins(
        &self,
        request: Request<GetAllAdminsRequest>,
    ) -> Result<Response<GetAllAdminsResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        info!("Get admins request from player {} received", username);

        let player = self.state().get_player_with_handling(&username).await?;

        if !player.is_admin() {
            return Err(Status::permission_denied("You are not an admin."));
        }

        let admin_ids: Vec<u64> = self
            .state()
            .permissions
            .admins
            .iter()
            .map(|r| r.clone())
            .collect();

        let mut admin_names: Vec<String> = Vec::new();

        for id in admin_ids {
            if let Some(username) = self.state().get_player_username(&id) {
                admin_names.push(username);
            } else {
                continue;
            }
        }

        info!("Get admins request from player {} completed", username);

        Ok(Response::new(GetAllAdminsResponse {
            usernames: admin_names,
        }))
    }
}
