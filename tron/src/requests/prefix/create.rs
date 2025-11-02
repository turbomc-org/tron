use crate::BridgeService;
use crate::bridge::{CreatePrefixRequest, CreatePrefixResponse};
use crate::models::prefix::Prefix;
use tonic::{Request, Response, Status};
use tracing::{error, info};

impl BridgeService {
    #[tracing::instrument(skip(self), fields(request = ?request.get_ref()))]
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

        self.send_message_to_player(
            &username,
            format!(
                "<gradient:#C724B1:#7A00FF><bold>✅ IDENTIFIER REGISTERED</bold></gradient>\n\
             <gray>Successfully registered the <color:{}>{}</color> identifier on the network.</gray>\n\
             <dark_gray>»</dark_gray> <gray>It is now available for players to acquire.</gray>",
                decompiled_prefix.color, decompiled_prefix.text
            ),
        )
        .await;

        info!("Create prefix request from player {} completed", username);

        Ok(Response::new(CreatePrefixResponse { success: true }))
    }
}
