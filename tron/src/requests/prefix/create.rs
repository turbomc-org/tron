use crate::BridgeService;
use crate::bridge::{CreatePrefixRequest, CreatePrefixResponse};
use crate::models::prefix::Prefix;
use tonic::{Request, Response, Status};
use tracing::{debug, error};

impl BridgeService {
    #[tracing::instrument]
    pub async fn handle_create_prefix(
        &self,
        request: Request<CreatePrefixRequest>,
    ) -> Result<Response<CreatePrefixResponse>, Status> {
        let inner_request = request.into_inner();
        let username = inner_request.username;
        let prefix = inner_request.prefix;

        debug!("Create prefix request from player {} received", username);

        if prefix.is_none() {
            error!(
                "Player {} tried to create a prefix without a prefix",
                username
            );
            return Err(Status::invalid_argument("Prefix cannot be empty"));
        }

        let _ = self.state.get_player_with_handling(&username).await?;
        let decompiled_prefix = Prefix::decompile(prefix.unwrap());

        if self
            .state
            .prefix_indexes
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
            .insert(&self.collections.prefixes, &self.state)
            .await
            .map_err(|e| {
                error!("Failed to insert prefix: {}", e);
                Status::internal("Failed to insert prefix")
            })?;

        debug!("Create prefix request from player {} completed", username);

        Ok(Response::new(CreatePrefixResponse { success: true }))
    }
}
