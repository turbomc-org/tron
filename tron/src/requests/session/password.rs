use crate::BridgeService;
use crate::bridge::{PlayerPasswordLoginRequest, PlayerPasswordLoginResponse};
use crate::utils::verify_password;
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_player_password_login(
        &self,
        request: Request<PlayerPasswordLoginRequest>,
    ) -> Result<Response<PlayerPasswordLoginResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let password = inner_request.password;

        let player = self.state().get_player_with_handling(&username).await?;

        if player.password.is_none() {
            return self.status(&username, Status::cancelled("You haven't registered password yet to password login. Please register yourself to password authentication by doing /register command.")).await;
        }

        let verify_password =
            match verify_password(&password, &player.password.as_ref().unwrap().clone()) {
                Ok(r) => r,
                Err(e) => {
                    return Err(Status::internal(e.to_string()));
                }
            };

        if !verify_password {
            return self
                .status(
                    &username,
                    Status::cancelled("Incorrect password you have few more tries to login."),
                )
                .await;
        }

        self.join_game(player).await;

        Ok(Response::new(PlayerPasswordLoginResponse { success: true }))
    }
}
