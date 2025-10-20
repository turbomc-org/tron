use crate::BridgeService;
use crate::bridge::{Prefix as GrpcPrefix, SendMessageRequest, SendMessageResponse};
use censor::{Censor, Sex, Standard, Zealous};
use tonic::{Request, Response, Status};
use tracing::error;

impl BridgeService {
    pub async fn handle_send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let inner_request = request.into_inner();
        let censor: Censor = Sex + Standard + Zealous;
        let username = inner_request.username;
        let message = censor.censor(&inner_request.message);
        let players_cache = &self.cache.active_players.clone();
        let players = &self.databases.players.clone();

        if !players_cache.contains_key(&username) {
            error!("Player {} not found in cache", username);
            return Err(Status::not_found(format!(
                "Player {} not found in cache",
                username
            )));
        }

        let player = self
            .cache
            .get_active_player(username.clone())
            .await
            .map_err(|err| {
                error!("Failed to get active player: {}", err.to_string());
                Status::internal(format!("Failed to get active player {}", username))
            })?;

        let mut grpc_prefix: Option<GrpcPrefix> = None;

        if let Some(prefix_id) = player.selected_prefix {
            match self.cache.get_prefix(prefix_id).await {
                Ok(Some(prefix_from_cache)) => {
                    grpc_prefix = Some(GrpcPrefix {
                        text: prefix_from_cache.prefix_text,
                        color: prefix_from_cache.display_color_hex,
                    });
                }
                Ok(None) => {
                    error!(
                        "Player {} has selected_prefix ID {} but it was not found in cache.",
                        username, prefix_id
                    );
                }
                Err(err) => {
                    error!("Failed to fetch prefix {} from cache: {}", prefix_id, err);
                }
            }
        }

        let response = SendMessageResponse {
            // The `prefix` field in the generated struct is an `Option`,
            // so we can assign our `grpc_prefix` variable directly.
            prefix: grpc_prefix,
            username,
            message,
        };

        Ok(Response::new(response))
    }
}
