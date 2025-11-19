use crate::config::messages::IDENTIFIER_REGISTERED;
use crate::models::prefix::Prefix;
use crate::{BridgeService, render};
use tonic::{Request, Response, Status};
use tracing::{error, info};
use tron_protos::{CreatePrefixRequest, CreatePrefixResponse};

impl BridgeService {
    pub async fn handle_create_prefix(
        &self,
        request: Request<CreatePrefixRequest>,
    ) -> Result<Response<CreatePrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let prefix = inner_request.prefix;

        info!("Create prefix request from player {} received", username);

        if prefix.is_none() {
            error!(
                "Player {} tried to create a prefix without a prefix",
                username
            );
            return Err(Status::invalid_argument("Prefix cannot be empty"));
        }

        let _ = self.state().get_player_with_handling(&username).await?;
        let decompiled_prefix = Prefix::decompile(prefix.unwrap());

        if self
            .state()
            .indexes
            .prefix
            .contains_key(&decompiled_prefix.text)
        {
            error!(
                "Player {} tried to create a prefix that already exists",
                username
            );
            return Err(Status::already_exists(format!(
                "Prefix with text {} already exists",
                decompiled_prefix.text
            )));
        }

        decompiled_prefix
            .insert(&self.collections().prefixes, &self.state())
            .await
            .map_err(|e| {
                error!("Failed to insert prefix: {}", e);
                Status::internal("Failed to insert prefix")
            })?;

        if let Err(e) = self
            .send_message(
                &username,
                render!(
                    IDENTIFIER_REGISTERED,
                    color = &decompiled_prefix.color,
                    text = &decompiled_prefix.text
                ),
            )
            .await
        {
            error!("Failed to send player message: {}", e);
        };

        info!("Create prefix request from player {} completed", username);

        Ok(Response::new(CreatePrefixResponse { success: true }))
    }
}
