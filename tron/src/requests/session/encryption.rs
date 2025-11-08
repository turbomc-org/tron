use crate::BridgeService;
use crate::bridge::PlayerEncryptionLoginRequest;
use crate::bridge::PlayerEncryptionLoginResponse;
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_player_encryption_login(
        &self,
        request: Request<PlayerEncryptionLoginRequest>,
    ) -> Result<Response<PlayerEncryptionLoginResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;

        if self.state().aliases.contains_key(&username) {
            if let Some(alias) = self.state().get_alias(&username) {
                return Ok(Response::new(PlayerEncryptionLoginResponse { alias }));
            } else {
                return Err(Status::internal(
                    "Key of alias was present in cache but record not found when requested",
                ));
            }
        }

        todo!("Implement encryption login")
    }
}
